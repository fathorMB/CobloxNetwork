"""Regression tests for the Coblox economic simulator.

Run from ``sim/``:

    python -m unittest discover -s tests -v
"""

from __future__ import annotations

import math
import pathlib
import re
import sys
import unittest

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1] / "tools"))

import protocol_hashes  # noqa: E402
import reward_rules  # noqa: E402
from dataclasses import replace
from fractions import Fraction

from coblox_sim import recommended as R
from coblox_sim import scenarios as S
from coblox_sim.election import (
    ChainStalled,
    ElectionState,
    CandidateNode,
    boundaries_to_admission_capture,
    boundaries_to_attrition_capture,
    derive_boundary,
    election_ticket,
    entropy_ids,
    genesis_set,
    minimum_pool_for_sustained_set,
)
from coblox_sim.emission import (
    NodePopulation,
    WorkChannel,
    fund_for_target_alpha,
    run_epoch,
)
from coblox_sim.params import (
    check_constraint_block,
    constraint_block_passes,
    feasible_c_values,
    term_limit_satisfiable,
)
from coblox_sim.population import build_contributors, lognormal, uniform


class TestModelValidation(unittest.TestCase):
    """GATE-MODEL-VALIDATED, as an assertion and not only as a printed table."""

    def test_reproduces_adr_007_arithmetic(self):
        points = S.s1_model_validation()
        self.assertEqual(len(points), 2)
        by_alpha = {round(p.alpha_requested, 2): p for p in points}
        self.assertAlmostEqual(by_alpha[1.00].captured_pct, 90.9091, places=3)
        self.assertAlmostEqual(by_alpha[0.10].captured_pct, 9.0909, places=3)
        self.assertEqual(round(by_alpha[1.00].captured_pct, 1), 90.9)
        self.assertEqual(round(by_alpha[0.10].captured_pct, 1), 9.1)

    def test_capped_fund_never_mints_more_than_the_cap(self):
        for E, F in ((1, 7), (3, 7), (11_000, 10**10), (9_999, 10**9 + 1)):
            res = run_epoch(
                NodePopulation(E, 0, 0), F, WorkChannel(total_microtokens=0)
            )
            self.assertLessEqual(res.existence_minted_total, F)
            self.assertEqual(
                res.existence_minted_total, res.existence_amount_per_node * E
            )
            self.assertEqual(
                res.existence_remainder_burned, F - res.existence_minted_total
            )

    def test_a_fleet_cannot_increase_total_emission(self):
        F = 10**10
        W = WorkChannel(total_microtokens=9 * 10**10)
        without = run_epoch(NodePopulation(900, 100, 0), F, W)
        with_fleet = run_epoch(NodePopulation(900, 100, 10_000), F, W)
        self.assertLessEqual(with_fleet.total_minted, without.total_minted)

    def test_phone_share_of_average_equals_alpha(self):
        """The identity the report is built on, checked over the whole grid."""

        for pt in S.s2_alpha_curve():
            self.assertAlmostEqual(pt.availability_only_over_average, pt.alpha, places=3)

    def test_fund_inverse_is_consistent(self):
        W = 90_000_000_000
        for a in (Fraction(1, 100), Fraction(3, 20), Fraction(1, 2)):
            F = fund_for_target_alpha(a, W)
            self.assertAlmostEqual(F / (F + W), float(a), places=6)


class TestConstraintBlock(unittest.TestCase):
    """GATE-CONSTRAINTS."""

    def test_recommended_set_passes(self):
        self.assertTrue(constraint_block_passes(S.s5_constraint_block()))

    def test_passes_with_the_term_limit_at_its_genesis_ceiling(self):
        self.assertTrue(constraint_block_passes(S.s5d_ratchet_still_valid()))

    def test_term_limit_of_three_or_less_is_unsatisfiable_at_every_set_size(self):
        for T in (1, 2, 3):
            sat, smallest = term_limit_satisfiable(T, 1, v_max=399)
            self.assertFalse(sat, f"T={T} should be unsatisfiable at every V")
            self.assertIsNone(smallest)
        for T in range(4, 17):
            sat, _ = term_limit_satisfiable(T, 1, v_max=399)
            self.assertTrue(sat, f"T={T} should be satisfiable for some V")

    def test_horizon_forces_the_term_limit(self):
        """T >= max(4, 3m), verified rather than quoted."""

        for m, smallest_T in S.s5c_horizon_coupling(m_max=6):
            self.assertEqual(smallest_T, max(4, 3 * m))

    def test_a_violating_combination_is_rejected(self):
        bad = replace(R.CONSENSUS, validator_churn_cap_seats=9)  # 3c = 27 is not < 27
        results = check_constraint_block(replace(R.RECOMMENDED, consensus=bad))
        self.assertFalse(constraint_block_passes(results))
        failed = {r.rule for r in results if not r.ok}
        self.assertIn("3 * c < V", failed)

    def test_monotonic_term_limit_is_enforced_against_an_active_document(self):
        shorter = replace(R.CONSENSUS, validator_max_consecutive_terms=8)
        results = check_constraint_block(
            replace(R.RECOMMENDED, consensus=shorter),
            active=R.CONSENSUS,
            active_activation_height=0,
            new_activation_height=10**9,
        )
        failed = {r.rule for r in results if not r.ok}
        self.assertIn("T_new >= T_active", failed)

    def test_activation_gap_is_enforced(self):
        results = check_constraint_block(
            R.RECOMMENDED,
            active=R.CONSENSUS,
            active_activation_height=0,
            new_activation_height=1,
        )
        failed = {r.rule for r in results if not r.ok}
        self.assertIn("activation_height spacing", failed)


class TestElectionDerivation(unittest.TestCase):
    def test_genesis_stagger_holds(self):
        g = genesis_set(R.CONSENSUS)
        self.assertEqual(g.member_count, R.CONSENSUS.V)
        counts: dict[int, int] = {}
        for s in g.seats:
            self.assertGreaterEqual(s.term_expiry_epoch, 1)
            self.assertLessEqual(s.term_expiry_epoch, R.CONSENSUS.T)
            counts[s.term_expiry_epoch] = counts.get(s.term_expiry_epoch, 0) + 1
        self.assertLessEqual(max(counts.values()), R.CONSENSUS.c)

    def test_unstaggerable_genesis_is_refused(self):
        bad = replace(R.CONSENSUS, validator_target_set_size=28)
        with self.assertRaises(ValueError):
            genesis_set(bad)

    def test_ticket_ordering_is_total_and_deterministic(self):
        chain = b"\x11" * 32
        seed = b"\x22" * 32
        pool = [CandidateNode(f"n-{i}", "honest", 10**9, 64) for i in range(500)]
        tickets = [election_ticket(chain, seed, n.key) for n in pool]
        self.assertEqual(len(set(tickets)), len(tickets))
        # Re-deriving gives the identical bytes.
        again = [election_ticket(chain, seed, n.key) for n in pool]
        self.assertEqual(tickets, again)

    def test_empty_pool_shrinks_the_set_and_eventually_stalls(self):
        d = S.s7_drought(0)
        self.assertGreater(d.boundaries_survived, 0)
        self.assertIn("validator_min_set_size", d.stall_reason)

    def test_minimum_pool_sustains_the_full_set(self):
        need = minimum_pool_for_sustained_set(R.CONSENSUS)
        d = S.s7_drought(need)
        self.assertEqual(d.final_member_count, R.CONSENSUS.V)
        self.assertIn("no stall", d.stall_reason)

    def test_cooldown_is_not_evadable_by_leaving_early(self):
        ev = S.s6d_cooldown_evasion()
        self.assertEqual(ev.voluntary_exit_absence, ev.parameter)
        self.assertEqual(ev.term_expiry_absence, ev.parameter)

    def test_contraction_floor_refuses_total_censorship(self):
        tot = S.s6b_at10_total_censorship(coalition=R.CONSENSUS.V // 3 + 1)
        self.assertIn("stalled", tot.outcome)

    def test_minimum_set_size_pins_selective_censorship(self):
        below = S.s6c_at10_selective_censorship(R.CONSENSUS.validator_min_set_size - 1)
        self.assertIn("pinned", below.outcome)
        at = S.s6c_at10_selective_censorship(R.CONSENSUS.validator_min_set_size)
        self.assertIn("whole set", at.outcome)

    def test_horizons(self):
        p = R.CONSENSUS
        self.assertEqual(boundaries_to_admission_capture(p.V, p.c), 3)
        self.assertEqual(boundaries_to_attrition_capture(p.V, p.V // 3 + 1), 3)


class TestDeterminism(unittest.TestCase):
    def test_draws_are_pure_functions_of_the_seed(self):
        a = [uniform("s", i) for i in range(50)]
        b = [uniform("s", i) for i in range(50)]
        self.assertEqual(a, b)
        self.assertNotEqual(a, [uniform("t", i) for i in range(50)])
        for v in a:
            self.assertGreaterEqual(v, 0.0)
            self.assertLess(v, 1.0)

    def test_population_is_reproducible(self):
        one = build_contributors(200, 28, S.ContributorProfile(), "seed")
        two = build_contributors(200, 28, S.ContributorProfile(), "seed")
        self.assertEqual([n.contribution_score for n in one],
                         [n.contribution_score for n in two])

    def test_lognormal_median_is_where_it_says(self):
        vals = sorted(lognormal("seed", i, 32.0, 1.5) for i in range(4000))
        median = vals[len(vals) // 2]
        self.assertGreater(median, 28.0)
        self.assertLess(median, 36.0)


class TestAttackTests(unittest.TestCase):
    def test_at07_passes_at_the_recommended_values(self):
        res = S.s4_at07()
        self.assertTrue(res.criterion_a)
        self.assertTrue(res.criterion_b)
        self.assertTrue(res.criterion_c)
        self.assertTrue(res.criterion_d)
        self.assertLessEqual(res.fleet_share_pct, 100 * R.X_DECLARED)

    def test_a_positive_availability_rate_breaks_criterion_a(self):
        bad = S.s4b_availability_channel_breaks_criterion_a()
        self.assertFalse(bad.criterion_a)

    def test_declared_x_bounds_capture_for_every_ratio(self):
        for N, H in ((10**4, 10**2), (10**4, 10**3), (10**5, 10**3), (10**6, 10**2)):
            share = R.ALPHA_SURVEILLANCE_BAND[1] * N / (N + H)
            self.assertLessEqual(share, R.X_DECLARED)


class TestGovernanceReach(unittest.TestCase):
    """REVIEW-011 RF-003 and RF-006, executed rather than accepted."""

    def test_three_parameters_are_frozen_by_the_rate_limit(self):
        frozen = {i.name for i in S.s9_legal_intervals() if i.frozen}
        self.assertEqual(
            frozen,
            {
                "validator_churn_cap_seats",
                "validator_cooldown_epochs",
                "validator_min_capture_epochs",
            },
        )

    def test_target_set_size_is_permanently_capped(self):
        self.assertEqual(S.s9b_max_reachable_v(), 36)
        self.assertGreater(R.CONSENSUS.validator_max_set_size, S.s9b_max_reachable_v())

    def test_term_limit_interval_never_goes_below_the_active_value(self):
        t = next(
            i for i in S.s9_legal_intervals()
            if i.name == "validator_max_consecutive_terms"
        )
        self.assertGreaterEqual(t.low, R.CONSENSUS.validator_max_consecutive_terms)
        self.assertLessEqual(t.high, R.BOUNDS.validator_max_consecutive_terms_max)

    def test_min_set_over_v_is_not_preserved_by_any_rule(self):
        steps = S.s10_min_set_ratio_erosion()
        self.assertTrue(all(e.constraint_block_passes for e in steps))
        self.assertAlmostEqual(steps[0].min_set_over_V, 2 / 3, places=3)
        self.assertAlmostEqual(steps[-1].min_set_over_V, 0.5, places=3)
        self.assertEqual(len(steps), 3, "reachable in two documents after genesis")

    def test_attrition_capture_completes_at_half_the_set_once_v_has_grown(self):
        results = {c.coalition_seats: c for c in S.s10b_censorship_at_eroded_ratio()}
        self.assertIn("whole set", results[18].outcome)  # 18/36 = exactly one half
        self.assertIn("pinned", results[13].outcome)


class TestLaunchRegime(unittest.TestCase):
    """REVIEW-011 RF-002."""

    def test_x_as_written_is_violated_below_the_usage_floor(self):
        ramp = {round(r.work_channel_microtokens / 1e6): r for r in S.s11b_usage_ramp()}
        launch = S.s11_at07_launch_regime(usage_fraction=0.0)
        self.assertTrue(launch.violates_x_as_written)
        self.assertGreater(launch.fleet_share_pct, 4 * launch.x_declared_pct)
        reference = S.s11_at07_launch_regime(usage_fraction=1.0)
        self.assertFalse(reference.violates_x_as_written)

    def test_the_absolute_diverted_amount_does_not_depend_on_usage(self):
        amounts = {r.absolute_diverted_microtokens for r in S.s11b_usage_ramp()}
        self.assertEqual(len(amounts), 1, "D = F * N/(N+H) contains no W")

    def test_usage_floor_is_where_the_band_becomes_holdable(self):
        below = S.s11_at07_launch_regime(usage_fraction=0.25)
        self.assertGreater(below.observed_alpha, R.ALPHA_SURVEILLANCE_BAND[1])


class TestQuorumIsNotPossession(unittest.TestCase):
    """REVIEW-014 RF-001: the relational rule bounds possession, not control."""

    def test_recommended_values_lose_quorum_at_forty_eight_percent(self):
        row = S.quorum_capture_threshold(27)
        self.assertEqual(row.min_set, 18)
        self.assertEqual(row.smallest_lawful_successor, 19)  # strict floor: not 18
        self.assertEqual(row.k_min_for_quorum, 13)
        self.assertAlmostEqual(row.k_min_fraction, 13 / 27, places=6)
        self.assertLess(row.k_min_fraction, 0.5)

    def test_threshold_is_far_below_two_thirds_at_every_size(self):
        for row in S.s12_quorum_capture_table():
            self.assertLess(
                row.k_min_fraction, 2 / 3,
                f"V={row.V}: quorum control needs {row.k_min_fraction:.3f} of the set",
            )

    def test_threshold_tends_to_four_ninths_from_above(self):
        rows = S.s12_quorum_capture_table((60, 600, 6000))
        fractions = [r.k_min_fraction for r in rows]
        self.assertEqual(fractions, sorted(fractions, reverse=True))
        self.assertLess(abs(fractions[-1] - 4 / 9), 0.002)
        for f in fractions:
            self.assertGreater(f, 4 / 9)

    def test_possession_threshold_is_the_two_thirds_one(self):
        row = S.quorum_capture_threshold(27)
        self.assertEqual(row.k_min_for_possession, 18)
        self.assertGreater(row.k_min_for_possession, row.k_min_for_quorum)

    def test_the_concrete_walk_reaches_quorum_then_possession(self):
        walk = S.s12b_quorum_capture_walk(V=27, coalition=13)
        self.assertFalse(walk[0].has_quorum)
        self.assertEqual(walk[1].set_size, 19)
        self.assertTrue(walk[1].has_quorum)
        self.assertFalse(walk[1].owns_set)
        self.assertTrue(walk[-1].owns_set)
        self.assertLessEqual(len(walk) - 1, 3, "three boundaries to full possession")

    def test_the_contraction_floor_is_strict(self):
        # The document's own methodological note: 27 goes to 19, not to 18.
        self.assertEqual(S.smallest_lawful_successor(27, 18), 19)
        self.assertGreater(3 * 19, 2 * 27)
        self.assertLessEqual(3 * 18, 2 * 27)


class TestToolConstantsTrackTheRegistry(unittest.TestCase):
    """The comparison constants in tools/ must not drift from the documents.

    [REVIEW-014] follow-up: the reward constant went stale and the tool reported
    a discrepancy that did not exist. A verification tool that cries wolf on a
    correct alignment teaches people to ignore it, which is worse than having no
    tool at all — and RF-007 existed precisely because nobody was looking.
    """

    def _registry(self) -> dict[str, str]:
        readme = (
            pathlib.Path(__file__).resolve().parents[2]
            / "docs" / "protocol" / "README.md"
        )
        text = readme.read_text(encoding="utf-8")
        wanted = {
            "parameter_set_hash": "enrollment_parameters",
            "policy_hash": "reward_policy",
            "hosting_rate_card_hash": "hosting_rate_card",
            "consensus_parameters_hash": "consensus_parameters",
        }
        found: dict[str, str] = {}
        for line in text.splitlines():
            m = re.match(r"\|\s*`(\w+)`\s*\|[^|]*\|\s*`(sha256:[0-9a-f]{64})`\s*\|", line)
            if m and m.group(1) in wanted:
                found[wanted[m.group(1)]] = m.group(2)
        return found

    def test_published_constants_match_the_protocol_registry(self):
        registry = self._registry()
        self.assertEqual(
            len(registry), 4, "could not parse all four document hashes from README.md"
        )
        for kind, published in registry.items():
            self.assertEqual(
                protocol_hashes.PUBLISHED[kind], published,
                f"tools/protocol_hashes.py is stale for {kind}",
            )

    def test_the_tool_reproduces_every_published_document_hash(self):
        registry = self._registry()
        bodies = {
            "enrollment_parameters": protocol_hashes.ENROLLMENT_BODY,
            "hosting_rate_card": protocol_hashes.HOSTING_BODY,
            "consensus_parameters": protocol_hashes.CONSENSUS_BODY,
            "reward_policy": protocol_hashes.reward_body("0"),
        }
        for kind, published in registry.items():
            self.assertEqual(protocol_hashes.document_hash(kind, bodies[kind]), published)

    def test_the_rejection_cases_all_hold(self):
        self.assertEqual(reward_rules.main(), 0)

if __name__ == "__main__":
    unittest.main()
