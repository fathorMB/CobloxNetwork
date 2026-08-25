"""Render the full simulation transcript.

    python -m coblox_sim              # everything
    python -m coblox_sim gates        # only the two before-submit gates
"""

from __future__ import annotations

import sys

from . import recommended as R
from . import scenarios as S
from .election import boundaries_to_admission_capture, boundaries_to_attrition_capture
from .emission import MICROTOKENS_PER_CREDIT
from .params import constraint_block_passes


def rule(title: str) -> None:
    print()
    print("=" * 78)
    print(title)
    print("=" * 78)


# --------------------------------------------------------------------------
# Gates
# --------------------------------------------------------------------------


def gate_model_validated() -> bool:
    rule("GATE-MODEL-VALIDATED — the model reproduces the arithmetic of [ADR-007]")
    print("Scenario: N = 10 000 emulated identities against H = 1 000 honest nodes.")
    print("Expected (verified independently by the Lead): 90,9 % at alpha=1, 9,1 % at alpha=0,1.")
    print()
    print(f"{'alpha':>7} {'observed alpha':>15} {'captured':>10} {'expected':>10} {'verdict':>9}")
    ok = True
    for pt in S.s1_model_validation():
        agrees = abs(pt.captured_pct - pt.expected_pct) < 0.01
        good = agrees and round(pt.captured_pct, 1) == round(pt.expected_pct, 1)
        ok = ok and good
        print(
            f"{pt.alpha_requested:>7.2f} {pt.observed_alpha:>15.6f} "
            f"{pt.captured_pct:>9.4f}% {pt.expected_pct:>9.4f}% {'PASS' if good else 'FAIL':>9}"
        )
    print()
    print("The figures are not the formula restated: the model mints one epoch node by")
    print("node under the ledger's own rule (amount = F // E, remainder discarded and")
    print("never minted) and then sums who received what.")
    print()
    print(f"GATE-MODEL-VALIDATED: {'PASS' if ok else 'FAIL'}")
    return ok


def gate_constraints() -> bool:
    rule("GATE-CONSTRAINTS — the recommended combination against the constraint block")
    p, b = R.CONSENSUS, R.BOUNDS
    print(f"parameter set: {R.RECOMMENDED.name}")
    print(f"V = {p.V}   T = {p.T}   c = {p.c}   m = {p.m}   "
          f"cooldown = {p.validator_cooldown_epochs}   min_set = {p.validator_min_set_size}")
    print()
    results = S.s5_constraint_block()
    for res in results:
        print("  " + res.line())
    ok = constraint_block_passes(results)
    print()
    print(f"  constraint block (genesis document): {'PASS' if ok else 'FAIL'}")

    print()
    print("After the [DEBT-010] ratchet pushes T to its genesis ceiling "
          f"({b.validator_max_consecutive_terms_max}):")
    pushed = S.s5d_ratchet_still_valid()
    for res in pushed:
        print("  " + res.line())
    ok2 = constraint_block_passes(pushed)
    print()
    print(f"  constraint block (T at ceiling): {'PASS' if ok2 else 'FAIL'}")

    print()
    print("Coupling 1, executed rather than quoted: is any (V, c) satisfiable for a")
    print("given term limit T at m = 1?  ledger.md claims T <= 3 is unsatisfiable at")
    print("every set size; brute force over V in [1, 399]:")
    print()
    print(f"  {'T':>3} {'satisfiable':>12} {'smallest V':>11}")
    coupling_ok = True
    for T, sat, smallest in S.s5b_term_limit_bruteforce():
        if T <= 3 and sat:
            coupling_ok = False
        if T >= 4 and not sat:
            coupling_ok = False
        print(f"  {T:>3} {str(sat):>12} {str(smallest):>11}")
    print()
    print(f"  T >= max(4, 3m) coupling reproduced: {'PASS' if coupling_ok else 'FAIL'}")

    print()
    print("Coupling 2: the declared capture horizon m bounds the term limit from below.")
    print(f"  {'m':>3} {'smallest T that admits any V':>30}")
    for m, smallest in S.s5c_horizon_coupling():
        print(f"  {m:>3} {str(smallest):>30}")

    gate = ok and ok2 and coupling_ok
    print()
    print(f"GATE-CONSTRAINTS: {'PASS' if gate else 'FAIL'}")
    return gate


# --------------------------------------------------------------------------
# The curve
# --------------------------------------------------------------------------


def section_curve() -> None:
    rule("The curve — alpha against defensibility and against meaning")
    print("The work channel is held at the reference usage (90 000 cr per one-day")
    print("epoch, 10 000 present nodes, one in five contributing). F follows from")
    print("alpha, which is the correct causal direction: alpha is observed, F is set.")
    print()
    print(f"{'alpha':>6} {'F (cr/ep)':>12} {'phone':>8} {'contrib':>9} "
          f"{'phone/avg':>10} {'cap H=100':>10} {'cap H=1e4':>10} {'cap H=1e5':>10}")
    for pt in S.s2_alpha_curve():
        print(
            f"{pt.alpha:>6.2f} {pt.fund_credits_per_epoch:>12,.0f} "
            f"{pt.availability_only_income_credits:>8.3f} "
            f"{pt.contributor_income_credits:>9.2f} "
            f"{pt.availability_only_over_average:>10.3f} "
            f"{100 * pt.capture_at_devnet:>9.2f}% "
            f"{100 * pt.capture_at_reference:>9.2f}% "
            f"{100 * pt.capture_at_scale:>9.2f}%"
        )
    print()
    print("Read the two middle columns together. `phone/avg` is the share of an average")
    print("node's income that an availability-only device receives, and it equals alpha")
    print("exactly, because the fund is split uniformly and the average node's income is")
    print("total emission over the same head count. The capture columns are")
    print("alpha * N/(N+H). Defensibility and meaning are therefore not two curves that")
    print("meet at an optimum: they are the same number read twice. A fleet is a fleet of")
    print("pretend phones, so whatever pays a phone pays a fleet member the same.")
    print()
    print("Consequence, stated because the model was built to look for a knee and there")
    print("is none: the capture curve is linear in alpha over the whole range. No value")
    print("of alpha is picked out by the arithmetic. The choice is a product decision and")
    print("the simulator's job is to price it, not to make it.")
    print()
    print(f"RECOMMENDATION: alpha = {R.ALPHA_TARGET}, surveillance band "
          f"[{R.ALPHA_SURVEILLANCE_BAND[0]}, {R.ALPHA_SURVEILLANCE_BAND[1]}], "
          f"X = {100 * R.X_DECLARED:.0f} %.")
    print("  Lower edge 0.10: below it an availability-only device receives under a")
    print("    tenth of an average node's income, and existence income stops being a")
    print("    floor and becomes a rounding line on a dashboard. That is the failure")
    print("    mode this spec was warned about, it appears in no capture number, and")
    print("    that is exactly why it has to be written down as a bound.")
    print("  Upper edge 0.20: above it more than a fifth of all issuance flows through")
    print("    the one channel a signature can enter, and the tolerance the project")
    print("    has to publish stops being defensible in a sentence.")
    print("  0.15 inside the band: nothing in the simulation prefers it to 0.12 or 0.18.")
    print("    It is the middle of a declared budget, and the operator owns the choice.")
    print(f"  X = {100 * R.X_DECLARED:.0f} % because capture is strictly below alpha for every N and H, so")
    print("    the band's upper edge is the only value of X provable by construction")
    print("    rather than by the particular N/H that happened to be on the test bench.")
    print()
    print("  X CARRIES A CONDITION AND MAY NOT BE PUBLISHED WITHOUT IT (REVIEW-011 RF-002):")
    print("    X = 20 % binds ABOVE the declared usage floor. Below it, alpha tends to 1")
    print("    by construction — see the drift section — and X is not an assertion at")
    print("    all; the quantity the network publishes there is the ABSOLUTE amount")
    print("    diverted per epoch. An X published without that condition is violated by")
    print("    about five times for the whole of the network's start-up, which is a")
    print("    public and checkable contradiction of a declared tolerance.")
    print()
    print("  ON THE POINT INSIDE THE BAND (REVIEW-011): because alpha is observed and is")
    print("    highest when the network is newest, the approach direction matters more")
    print("    than the point. The network crosses the whole band from ABOVE during")
    print("    start-up, so any margin the operator wants should be taken on the UPPER")
    print("    edge — the one dual to X, and the one tested first.")
    print()
    print("  AND THE LOWER EDGE IS A PRODUCT CHOICE DRESSED AS A MEASUREMENT, said")
    print("    plainly rather than nearly (REVIEW-011 RF-005): no simulated quantity")
    print("    selects 0.10. It is a judgement about meaning, not a result. It is worth")
    print("    writing down precisely because it protects something no security number")
    print("    can see, and it would be the first bound to disappear under tuning")
    print("    pressure — but it must not be published as though the model produced it.")


def section_dilution() -> None:
    rule("What lowering alpha does NOT buy")
    print("An honest availability-only node earns F/E. A fleet of N inflates E, so the")
    print("honest node keeps a fraction H/(N+H) of what it would have earned. That")
    print("factor contains no alpha at all.")
    print()
    print(f"{'N':>8} {'H':>8} {'honest keeps':>14} {'depends on alpha':>18}")
    for N, H in ((10_000, 100), (10_000, 1_000), (10_000, 10_000), (10_000, 100_000)):
        keep = H / (N + H)
        print(f"{N:>8,} {H:>8,} {100 * keep:>13.2f}% {'no':>18}")
    print()
    print("Lowering alpha reduces the headline capture percentage — a number about the")
    print("attacker — and reduces the honest phone's income by exactly the same factor")
    print("whether or not an attack is under way. It does not reduce the honest node's")
    print("loss ratio under attack by one part in a thousand.")
    print()
    print("[ADR-007] measured the attacker-facing quantity and is correct about it. This")
    print("is the user-facing quantity and it is governed by N/(N+H) alone. The practical")
    print("consequence is that the defence budget is better spent on what actually")
    print("separates a phone from a fleet member — the Argon2id entry floor, the")
    print("availability threshold, and address-diversity limits aggregated by routed")
    print("prefix (threat-model.md 6.1, lever 3) — than on shaving alpha. Contested")
    print("assumption, raised here rather than buried, and confirmed at GATE-SECREVIEW.")
    print()
    print("THE BAND AND X ARE DECLARED IN TWO DIFFERENT WORLDS (REVIEW-011 RF-005), and")
    print("the project will publish both, so the incoherence has to be published too.")
    print()
    print("  The band's lower edge of 0.10 protects the meaning of existence income and")
    print("  is computed WITH NO ADVERSARY PRESENT: a phone earns 0.15 of an average")
    print("  node's income at the recommended value. X = 20 % declares an adversary")
    print("  present and tolerated. In the world where X is declared, the phone does not")
    print("  earn 0.15 of an average node's income — at the AT-07 bench it earns 0.0157")
    print("  cr per epoch instead of 1.588, two orders of magnitude less.")
    print()
    print("  The two are not in arithmetic contradiction; they measure different things.")
    print("  They are incoherent as PROMISES. The lower edge protects a quantity that")
    print("  the tolerated attack destroys by two orders of magnitude, so the band")
    print("  cannot be published on its own — it goes out with this paragraph attached.")


def section_drift() -> None:
    rule("alpha is observed, not set — and it is highest when the network is newest")
    print("`ledger.md` says it in as many words: the fraction is 'an observed ratio")
    print("between channels, not a knob'. The knob is F, so alpha drifts with usage.")
    print()
    print(f"Fund held fixed at F = "
          f"{R.REWARD.existence_fund_microtokens_per_epoch / MICROTOKENS_PER_CREDIT:,.0f} cr per epoch.")
    print()
    print(f"{'usage':>7} {'W (cr/ep)':>12} {'alpha':>8} {'F to hold 0.15':>16} {'phone (cr)':>11}")
    for d in S.s3_alpha_drift():
        print(
            f"{d.usage_fraction_of_reference:>7.2f} {d.work_channel_credits:>12,.0f} "
            f"{d.alpha_with_fixed_fund:>8.4f} {d.fund_credits_to_hold_target:>16,.0f} "
            f"{d.availability_only_income_credits:>11.3f}"
        )
    print()
    print("At genesis W is near zero, so alpha is near one whatever F is. Holding the")
    print("surveillance band from block one would require F near zero — no existence")
    print("income at all at launch, on the one day the promise most needs to be visible.")
    print()
    print("The band therefore binds from a declared usage floor. Recommended trigger:")
    print("the band applies once the work channel reaches 25 % of the reference usage.")
    print("Below that floor the network publishes the ABSOLUTE amount diverted instead")
    print("of the ratio, which is the honest quantity there: 91 % of a very small")
    print("emission is a very small emission. This is a governance rule for the fund,")
    print("not a protocol rule, and it needs no schema field.")
    print()
    print("What that means for the declared tolerance, spelled out because leaving it")
    print("implicit is what REVIEW-011 RF-002 found: X = 20 % is an assertion only above")
    print("the usage floor. Below it the assertion is replaced, not weakened, by the")
    print("absolute diverted amount D = F * N/(N+H) per epoch. D does not depend on the")
    print("work channel at all, so the only lever on it is F itself — a governed value")
    print("now held under a genesis ceiling and a per-document rate limit by [ADR-011]")
    print("(see the fund-shape section), and free to move anywhere below that ceiling")
    print("one bounded step at a time. At launch, F should be sized to the honest node")
    print("count actually present rather than to the reference network, so that D scales")
    print("with the network instead of with a population that has not arrived yet.")


# --------------------------------------------------------------------------
# Fund shape
# --------------------------------------------------------------------------


def section_fund_shape() -> None:
    rule("The shape of the fund: the cap per epoch and the split")
    print("The split is NOT free. `ledger.md` already fixes it as a validity rule:")
    print()
    print("    E > 0 ;  amount_microtokens = F / E    (integer division, remainder")
    print("    discarded, not carried forward, and never minted)")
    print()
    print("so the split is uniform among the nodes that met the threshold, and any")
    print("weighted variant is a protocol change and outside this spec's scope.")
    print("Confirmed rather than merely inherited, because the spec asked for the")
    print("interaction between the split criterion and capture by numerosity:")
    print()
    print("  * A uniform split maximises capture by numerosity — a fleet of N takes")
    print("    N/(N+H) of the fund — and that is the cost of the choice, stated plainly.")
    print("  * A split weighted by demonstrated contribution would reduce it, and would")
    print("    also destroy the thing being paid for. The only Sybil-hard weights the")
    print("    network possesses are storage and compute, which the work channel already")
    print("    pays per unit. Weighting the fund by them makes existence income a second,")
    print("    worse-denominated copy of work compensation, and an availability-only")
    print("    device — the project's characteristic device — receives a weight of about")
    print("    zero. Capture would fall to nearly nothing, and so would the promise on")
    print("    the front page of PROJECT.md. That is the trap this spec was warned")
    print("    about, reached by a route that looks like prudence.")
    print("  * There is no third option, and this is the honest part: a weight that was")
    print("    neither uniform nor a contribution measure would have to be Sybil-hard")
    print("    and not already paid for, and the network has no such quantity. None was")
    print("    found.")
    print()
    print("DECISION: uniform split, as the protocol already requires, with a per-epoch")
    print("cap F set to hold alpha inside its surveillance band. F is a governed value")
    print(f"in `reward_policy`; the genesis value is "
          f"{R.REWARD.existence_fund_microtokens_per_epoch:,} microtokens per one-day")
    print(f"epoch ({R.REWARD.existence_fund_microtokens_per_epoch / MICROTOKENS_PER_CREDIT:,.0f} cr), "
          f"which is alpha = {R.ALPHA_TARGET} at the reference usage.")
    print()
    print("Governance rule for F, applied per reward epoch and published:")
    print("  1. observe alpha = existence emission / total emission over the last epoch;")
    print("  2. if alpha is inside the band, leave F unchanged;")
    print("  3. if alpha leaves the band, move F toward the target by at most 25 % per")
    print("     document, the same 5/4 discipline the election parameters already use;")
    print("  4. below the usage floor — work channel under 25 % of reference — suspend")
    print("     the band and publish the absolute diverted amount instead.")
    print()
    print("WHAT THIS RULE IS AND WHAT IT IS NOT (REVIEW-011 RF-001, part 1,")
    print("CLOSED BY [ADR-010], [ADR-011], [SPEC-009] AND [SPEC-011]):")
    print()
    print("  This section used to declare three of the things this study relies on to")
    print("  be VALUES AND PRACTICE that no protocol document enforced. Two of the")
    print("  three are now validity rules, applied on acceptance and exhibited")
    print("  rejecting: a document that violates one is INVALID, not merely unwise.")
    print()
    print("    * availability_microtokens_per_unit = 0 — now a RULE ([ADR-010]). A")
    print("      positive tariff is rejected on acceptance, so the counter-example")
    print("      measured in the AT-07 section is a state no conformant network can")
    print("      reach. Enforced in `RewardPolicy::check_internal`.")
    print("    * the per-epoch cap F — now bounded ([ADR-011]). RewardBounds carries")
    print("      existence_fund_microtokens_per_epoch_max, and F above the genesis")
    print("      ceiling is rejected. The old jump from 15 882 cr to 2^60 microtokens")
    print("      in one lawful document is no longer lawful.")
    print("    * the 5/4 discipline on F — now a RULE ([ADR-011]), and applied to")
    print("      every one of the thirteen reward parameters rather than to F alone.")
    print("      It binds in BOTH directions: neither a rise nor a fall may exceed")
    print("      the genesis ratio in one document, and consecutive activations must")
    print("      be spaced by reward_parameter_min_activation_gap_blocks.")
    print()
    print("  What remains a VALUE and not a rule is the genesis content of")
    print("  RewardBounds itself. The bounds object is configuration in the signed")
    print("  distribution, nothing on-chain constrains it, and a degenerate one would")
    print("  weaken the rules it carries — which is why the crate validates the anchor")
    print("  before trusting it, in `light_client::authenticate_reward_policy`, and")
    print("  refuses a degenerate change ratio rather than applying it vacuously.")
    print("  Choosing the launch values — F_max above all — is still open work.")
    print()
    print("  Criteria (a) and (c) of the [ADR-007] metric are therefore no longer")
    print("  conditional on the active reward policy volunteering to respect them:")
    print("  they rest on acceptance-time rules for (a), and for (c) on a bounded F")
    print("  plus a bounded rate of change. They remain conditional on the genesis")
    print("  bounds being sensibly chosen, which is a smaller and visible assumption")
    print("  than the one this section used to record.")


# --------------------------------------------------------------------------
# Attack tests
# --------------------------------------------------------------------------


def section_at07() -> None:
    rule("AT-07 — emulated fleet against existence income: numeric verdict")
    res = S.s4_at07()
    print(f"H = {res.honest_nodes} honest nodes, N = {res.fleet_nodes:,} emulated on one host, "
          f"alpha target {R.ALPHA_TARGET}, X declared {res.x_declared_pct:.1f} %")
    print()
    print(f"  (a) total emission without fleet : {res.total_emission_without_fleet:,} microtokens")
    print(f"      total emission with fleet    : {res.total_emission_with_fleet:,} microtokens")
    print(f"      criterion (a) not increased  : {'PASS' if res.criterion_a else 'FAIL'}")
    print(f"  (b) fleet share of emission      : {res.fleet_share_pct:.3f} %  "
          f"(<= X = {res.x_declared_pct:.1f} %) -> {'PASS' if res.criterion_b else 'FAIL'}")
    print(f"  (c) fleet storage/compute credit : {res.fleet_storage_compute_credits} -> "
          f"{'PASS' if res.criterion_c else 'FAIL'}")
    print(f"  (d) fleet validator seats        : {res.fleet_validator_seats} -> "
          f"{'PASS' if res.criterion_d else 'FAIL'}")
    print("      (d) holds by the eligibility rule and not by luck: `contribution_score`")
    print("      counts availability evidence as zero, so a fleet that only signs has a")
    print("      score of 0 and fails eligibility condition 3 at any positive threshold.")
    print()
    print(f"  honest availability-only income without fleet: "
          f"{res.honest_income_without_fleet / MICROTOKENS_PER_CREDIT:,.4f} cr/epoch")
    print(f"  honest availability-only income with fleet   : "
          f"{res.honest_income_with_fleet / MICROTOKENS_PER_CREDIT:,.4f} cr/epoch")
    print("  (the devnet income level is an artifact of H = 100 sharing a fund sized for")
    print("   10 000 nodes; the ratio below is the quantity to read, not the level)")
    print(f"  the honest node keeps "
          f"{100.0 * res.honest_income_with_fleet / res.honest_income_without_fleet:.2f} % "
          f"of its income")
    print()
    print(f"AT-07 at the reference usage regime: {'PASS' if res.passed else 'FAIL'}")
    print()
    print("AT-07 IN THE REGIME IT WILL ACTUALLY BE RUN IN (REVIEW-011 RF-002)")
    print("The test is scheduled on a devnet, and a devnet has no usage: W is near zero,")
    print("so alpha is near one whatever F is. The same bench, along the usage ramp:")
    print()
    print(f"{'W (cr/ep)':>12} {'alpha':>8} {'fleet share':>12} {'diverted (cr/ep)':>18} "
          f"{'(c) as written':>16}")
    for r in S.s11b_usage_ramp():
        print(f"{r.work_channel_microtokens / MICROTOKENS_PER_CREDIT:>12,.0f} "
              f"{r.observed_alpha:>8.4f} {r.fleet_share_pct:>11.2f}% "
              f"{r.absolute_diverted_microtokens / MICROTOKENS_PER_CREDIT:>18,.0f} "
              f"{'VIOLATED' if r.violates_x_as_written else 'held':>16}")
    print()
    print("  Criterion (c) as literally written is violated by about five times for the")
    print("  whole start-up period — which is precisely when a fleet is cheapest and the")
    print("  network is most exposed. The damage is the public, checkable contradiction")
    print("  of a tolerance the project declared, in a project whose front page says")
    print("  'super-secure' and which has already had to withdraw one metric.")
    print()
    print("  And note the fourth column, because it corrects something this study said")
    print("  loosely in its first pass. The absolute diverted amount does NOT move along")
    print("  the ramp: D = F * N/(N+H) contains no W. Saying '99 % of a tiny emission is")
    print("  a tiny emission' was only true if F is also small, and F is a governance")
    print("  choice, not a consequence of low usage. With the genesis F sized for 10 000")
    print("  nodes, a launch fleet diverts about 15 725 cr per epoch — nearly the whole")
    print("  fund — while the honest hundred share the remainder. The absolute criterion")
    print("  is therefore only honest if F at launch is sized to the honest node count")
    print("  actually present. Sizing F at launch remains a governance choice (see the")
    print("  fund-shape section); what RewardBounds now supplies is the ceiling it")
    print("  cannot exceed and the rate limit that makes a later move to that ceiling")
    print("  an observable process instead of one document.")
    print()
    print(f"AT-07 VERDICT: PARTIALLY COVERED.")
    print("  * reference usage regime ... PASS on all four criteria (above)")
    print("  * launch regime ............ criterion (c) as written FAILS; the applicable")
    print("    criterion there is the absolute one, and X is not an assertion below the")
    print("    usage floor")
    print("  The matrix therefore reports AT-07 as partially covered, not as passed,")
    print("  until X carries its usage condition and the launch-regime verdict is run")
    print("  against the absolute criterion.")
    print()
    print("Counter-example, and the reason availability_microtokens_per_unit must be 0:")
    bad = S.s4b_availability_channel_breaks_criterion_a()
    print(f"  with a non-zero availability work rate, total emission goes from")
    print(f"  {bad.total_emission_without_fleet:,} to {bad.total_emission_with_fleet:,} microtokens")
    print(f"  criterion (a): {'PASS' if bad.criterion_a else 'FAIL'} — the fleet mints.")
    print()
    print("  `work_compensation` with work_kind = 'availability' is a per-node amount")
    print("  with no cap. `RewardPolicyBody` carries the rate and nothing in the document")
    print("  forbids a positive value, so N emulated identities would increase total")
    print("  emission and criterion (a) of the [ADR-007] metric would fail on the first")
    print("  epoch. This is not a defect in the rule and needs no ADR: the schema field")
    print("  stays and the VALUE must be zero. It is recorded here because a zero that")
    print("  is not written down is a zero somebody later raises 'just a little'.")


def section_at10() -> None:
    rule("AT-10 — election capture: numeric verdict on all three configurations")
    p = R.CONSENSUS
    print(f"V = {p.V}, c = {p.c}, T = {p.T}, m = {p.m}, "
          f"validator_min_set_size = {p.validator_min_set_size}")
    print(f"admission horizon  ceil((V/3)/c) = {boundaries_to_admission_capture(p.V, p.c)} boundaries "
          "(tunable through c)")
    print(f"attrition horizon  ceil(log(V/k)/log(3/2)) at k just above V/3 = "
          f"{boundaries_to_attrition_capture(p.V, p.V // 3 + 1)} boundaries "
          "(fixed; no parameter moves it)")
    print()
    print("Configuration 1 — seed grinding by the proposer of the last entropy block")
    print("The adversary supplies real storage and compute just above the threshold —")
    print("the threat model's own correction, since uptime contributes zero to the")
    print("score — and grinds only at boundaries where it holds the proposal slot,")
    print("because only a proposer can grind. G resamples of the last entropy block.")
    print()
    print(f"{'ratio':>10} {'adv cand':>9} {'hon cand':>9} {'fills G=1':>10} "
          f"{'fills G=%d' % S.GRINDING_ATTEMPTS:>12} {'to 1/3 G=1':>11} {'to 1/3 grind':>13} "
          f"{'final seats':>12}")
    grinding_gain_bounded = True
    reached_before_m = False
    for g in S.s6a_at10_grinding():
        if g.adversary_fills > g.adversary_fills_no_grinding + p.c * 50:
            grinding_gain_bounded = False
        for r in (g.epochs_to_one_third, g.epochs_to_one_third_no_grinding):
            if r is not None and r < p.m:
                reached_before_m = True
        print(
            f"{g.ratio_label:>10} {g.adversary_candidates:>9} {g.honest_candidates:>9} "
            f"{g.adversary_fills_no_grinding:>10} {g.adversary_fills:>12} "
            f"{str(g.epochs_to_one_third_no_grinding or 'never'):>11} "
            f"{str(g.epochs_to_one_third or 'never'):>13} "
            f"{f'{g.final_adversary_seats}/{g.final_member_count}':>12}"
        )
    print()
    print(f"  grinding gain bounded by the churn cap .................... "
          f"{'PASS' if grinding_gain_bounded else 'FAIL'}")
    print(f"  no capture faster than the declared m = {p.m} boundaries ........ "
          f"{'PASS' if not reached_before_m else 'FAIL'}")
    print()
    print("  Configuration 1 PASSES what the rule claims: grinding yields bias and never")
    print("  choice, and 128 resamples move the fill count by less than one boundary's")
    print("  churn cap across 50 boundaries.")
    print()
    print("  Configuration 1 FAILS the pass criterion AT-10 currently states — 'the")
    print("  attacker does not reach 1/3 within 50 epochs' — at N/H = 1 and N/H = 10.")
    print("  This is not a tuning failure and no parameter set repairs it. Time to one")
    print("  third by admission is ceil((V/3)/c) boundaries at best for the adversary,")
    print("  and the constraint block forces (V/3)/c >= m. The literal criterion is")
    print("  therefore the requirement m >= 50, which forces T >= 3m = 150 and")
    print("  c <= V/150: a set of at least 150 validators rotating one seat per boundary")
    print("  with terms of 150 boundaries, about three years at a seven-day boundary.")
    print("  That is not a network anyone can operate, and it is the exact opposite of")
    print("  the [DEBT-010] instruction to keep the term limit as tight as tolerated.")
    print()
    print("  The threat model has already corrected one AT-10 criterion for being wrong")
    print("  rather than merely unmet. This is the second, and the security review has")
    print("  since named what the two have in common, which is more useful than either")
    print("  correction: BOTH ARE ABSOLUTE STATEMENTS ABOUT AN EMERGENT QUANTITY,")
    print("  written before the rule that produces that quantity existed, and NEITHER")
    print("  NAMES A RULE. A criterion of that shape cannot be checked when it is")
    print("  written — only when a simulation contradicts it, and by then it")
    print("  contradicts it against whoever implemented the thing. The defect is in how")
    print("  the test was written, not in the individual criteria.")
    print()
    print("  The replacement proposed below does not have that shape: every clause is an")
    print("  inequality against a published parameter, and every clause names the rule")
    print("  that imposes it. AGENT-007 has adopted the correction and added the")
    print("  corresponding writing convention to threat-model.md. The substitution")
    print("  itself REDUCES A PUBLIC CLAIM and therefore goes to the operator; it is")
    print("  recorded, not applied unilaterally. No protocol rule changes either way:")
    print("  the rule guarantees that the time to one third is at least m boundaries and")
    print("  that every one of them publishes its drift, and it delivers exactly that.")

    print()
    print("Configuration 2a — total censorship, refused by the contraction floor")
    tot = S.s6b_at10_total_censorship(coalition=p.V // 3 + 1)
    print(f"  coalition holds {tot.coalition_seats} of {p.V} seats "
          f"({100.0 * tot.coalition_seats / p.V:.1f} %)")
    print(f"  member count by boundary: {tot.boundaries}")
    print(f"  outcome: {tot.outcome}")
    print("  configuration 2a: PASS — the coalition obtains a halt, never the set.")

    print()
    print("Configuration 2b — selective censorship, the real vector")
    conf2b_pass = True
    for k in (p.V // 3 + 1, p.validator_min_set_size - 1, p.validator_min_set_size,
              p.validator_min_set_size + 1):
        sel = S.s6c_at10_selective_censorship(k)
        captured = "whole set" in sel.outcome
        if captured and k < p.validator_min_set_size:
            conf2b_pass = False
        print(f"  k = {k:>2} ({100.0 * k / p.V:>4.1f} % of V): sizes {sel.boundaries}")
        print(f"           {sel.outcome}")
        print(f"           continuous-form prediction: {sel.predicted_continuous} boundaries")
    print(f"  configuration 2b: {'PASS' if conf2b_pass else 'FAIL'}")
    print()
    print(f"  The result worth naming: with validator_min_set_size = {p.validator_min_set_size} the attrition")
    print("  path is closed for every coalition smaller than that. `ledger.md` credits")
    print("  the contraction floor alone and concludes the effective capture threshold")
    print("  is 'just above one third'. That is exactly true of the floor taken by")
    print(f"  itself; setting the minimum set size to 2V/3 = {p.validator_min_set_size} raises it to two")
    print("  thirds, at which point the BFT safety assumption has already failed and no")
    print("  set-composition rule was ever claiming to help. The floor does what the")
    print("  document says; the minimum does more than the document credits it with.")
    print("  This is a security claim produced by a tuning spec and it is the single")
    print("  most important item for AGENT-007 at GATE-SECREVIEW.")
    print()
    print("  Its price is liveness, and it is paid in the drought table below: the set")
    print("  may not lawfully shrink below 18, so three consecutive boundaries with an")
    print("  empty fill pool stall the chain instead of five.")
    print()
    print("  AND IT IS A PROPERTY OF THIS COMBINATION, NOT OF THE RULES (REVIEW-011")
    print("  RF-003). The property depends on the RATIO min_set/V, and no rule preserves")
    print("  that ratio: the constraint block requires only 0 < min_set <= V.")
    print("  validator_min_set_size_min stops the minimum being lowered; nothing stops V")
    print("  being raised. The governance-reach section below measures the path — two")
    print("  lawful documents, about fourteen days — and what it costs:")
    for e in S.s10_min_set_ratio_erosion():
        print(f"    document {e.document}: V={e.V:>2} T={e.T:>2} min_set={e.min_set} "
              f"-> min_set/V = {e.min_set_over_V:.3f}   constraint block: "
              f"{'PASS' if e.constraint_block_passes else 'FAIL'}")
    print()
    print(f"{'  coalition':>12} {'sizes':>18}  outcome at V = 36, min_set = 18")
    for c in S.s10b_censorship_at_eroded_ratio():
        short = "WHOLE SET" if "whole set" in c.outcome else "pinned"
        print(f"{c.coalition_seats:>12} {str(c.boundaries):>18}  {short} "
              f"({100.0 * c.coalition_seats / 36:.1f} % of V)")
    print()
    print("  At V = 36 the attrition threshold is exactly HALF the set, where BFT safety")
    print("  has NOT failed and the network still believes it has a guarantee. The")
    print("  property does not degrade gradually — it moves below the line that made it")
    print("  harmless. The claim must therefore be stated as: closed below 2V/3 AT THE")
    print("  RECOMMENDED COMBINATION; in general closed only below validator_min_set_size,")
    print("  a quantity governance may leave behind while V grows. `ledger.md`'s")
    print("  conclusion — effective threshold just above one third — is the correct")
    print("  figure for the worst governable case and must NOT be raised to two thirds")
    print("  until a rule preserves the ratio. That rule (3 * min_set >= 2 * V) is a")
    print("  protocol change, out of scope here, and the Lead's ADR to open.")
    print()
    print("  Note on the discrete correction: the continuous formula predicts fewer")
    print("  boundaries than the simulation because the contraction floor is strict —")
    print("  a set of 27 may shrink to 19, not to 18. The measured figure is the one to")
    print("  quote, and it is never smaller than the formula's.")

    print()
    print("Configuration 3 — cooldown evasion")
    ev = S.s6d_cooldown_evasion()
    print(f"  validator_cooldown_epochs = {ev.parameter}")
    print(f"  absence after a term expiry     : {ev.term_expiry_absence} epochs")
    print(f"  absence after a voluntary exit  : {ev.voluntary_exit_absence} epochs")
    ok3 = ev.voluntary_exit_absence == ev.term_expiry_absence == ev.parameter
    print(f"  configuration 3: {'PASS' if ok3 else 'FAIL'} — eligibility condition 5 in the")
    print("  'left a seat for any reason whatsoever' form makes the two measurements")
    print("  equal, which is the whole point of the test. Under the earlier form limited")
    print("  to term expiry the voluntary exit would have measured one epoch.")

    print()
    print("AT-10 VERDICT, criterion by criterion rather than as one word:")
    print("  * grinding bounded by the churn cap and not by the seed ............ PASS")
    print("  * total censorship yields a halt, never a set of the coalition's own  PASS")
    print("  * selective censorship pinned at validator_min_set_size below 2V/3 . PASS")
    print("  * cooldown not evadable by leaving one epoch early ................. PASS")
    print("  * composition drift light-client computable at every boundary ...... PASS")
    print("      by construction: both ValidatorSet documents are held in clear and")
    print("      filled_count is checked against validator_churn_cap_seats")
    print("  * 'does not reach 1/3 within 50 epochs', all three N/H ratios ...... FAIL")
    print("      at N/H = 1 and N/H = 10, and not repairable by any operable")
    print("      parameter set; see the configuration 1 note.")
    print()
    print("The honest summary: the rule delivers everything it claims, and this one test")
    print("criterion asks for something the rule never claimed. Tuning m above 3 would")
    print("buy nothing in any case, because the attrition horizon is fixed at three")
    print("boundaries and a rule is only as strong as its weakest path. That is why")
    print("m = 3 and validator_min_capture_epochs_min = 3.")


# --------------------------------------------------------------------------
# Couplings
# --------------------------------------------------------------------------


def section_couplings() -> None:
    rule("The three couplings, simulated together — where the network stops with no adversary")
    p = R.CONSENSUS
    print("Coupling 1: cooldown, the eligibility threshold, and the pool.")
    print()
    print(f"{'threshold':>10} {'contributors':>13} {'eligible':>9} {'pool needed':>12} "
          f"{'willingness needed':>19}")
    for r in S.s7b_eligibility_pool((128, 256, 512, 1_024, 4_096, 16_384, 65_536, 262_144)):
        w = f"{r.willingness_needed_pct:.2f} %" if r.eligible else "unsatisfiable"
        print(f"{r.threshold_units:>10,} {r.contributors:>13,} {r.eligible:>9,} "
              f"{r.minimum_pool_required:>12} {w:>19}")
    print()
    print(f"At the recommended threshold of {R.REWARD.validator_eligibility_threshold_units} units "
          "the eligibility bar is not the binding")
    print("constraint — willingness to stand is. The arithmetic minimum is")
    print(f"{S.minimum_pool_for_sustained_set(p)} distinct eligible nodes filing candidacies "
          f"to hold {p.V} seats:")
    print(f"  {p.V} seated + ceil(V/T) = {-(-p.V // p.T)} entering cooldown per boundary for "
          f"{p.validator_cooldown_epochs} boundaries + {-(-p.V // p.T)} free to refill.")
    print()
    print("The drought: no adversary at all, only a finite pool of nodes willing to")
    print("stand. The same nodes re-file every epoch cooldown allows, which is what")
    print("makes cooldown bite; a supply of brand-new nodes each boundary would never")
    print("meet the cooldown and would hide the coupling this scenario exists to show.")
    print()
    print(f"{'standing pool':>14} {'cooldown':>9} {'boundaries survived':>20} "
          f"{'final size':>11}  stall reason")
    for pool in (0, 6, 12, 18, 24, 30, 33, 36, 40):
        d = S.s7_drought(pool)
        print(f"{pool:>14} {d.cooldown:>9} {d.boundaries_survived:>20} "
              f"{d.final_member_count:>11}  {d.stall_reason}")
    print()
    print("Cooldown sensitivity at a standing pool of 33 — just short of the arithmetic")
    print("minimum, which is where the parameter can still change the answer:")
    print()
    print(f"{'cooldown':>9} {'boundaries survived':>20} {'final size':>11}  stall reason")
    for cd in (1, 2, 3, 5, 9):
        d = S.s7_drought(33, cooldown=cd)
        print(f"{cd:>9} {d.boundaries_survived:>20} {d.final_member_count:>11}  {d.stall_reason}")
    print()
    print("WHERE THE NETWORK STOPS WITHOUT AN ADVERSARY, as a number:")
    print("  * below a standing pool of 30 the chain stalls — in 3 boundaries at a pool")
    print("    of zero, in 11 boundaries at a pool of 24;")
    print("  * between 30 and 35 it survives but settles below its target size;")
    print("  * at 36, the arithmetic minimum, it holds all 27 seats.")
    print("  That is a participation threshold, not a parameter value, and it is the")
    print("  quantity an operator should watch. At the reference network it means about")
    print("  3 % of contributors need to be willing to stand.")
    print()
    print("  The cooldown table is the coupling the spec named: at a pool of 33 a")
    print("  cooldown of 5 or 9 stalls the chain after 14 boundaries with no adversary")
    print("  anywhere, purely because the leavers cannot return fast enough to refill")
    print("  the seats the term limit keeps emptying. Cooldown is also the one election")
    print("  quantity whose increase helps an adversary — censoring one candidacy for")
    print("  one epoch removes that node for 1 + cooldown epochs. Both arguments point")
    print("  the same way, which is why the recommendation is 2 and not the maximum the")
    print("  constraint block would allow.")

    print()
    print("Coupling 2: validator_max_consecutive_terms_max, the [DEBT-010] residual guard.")
    print()
    print(f"{'T':>4} {'seats vacated/boundary':>23} {'minimum pool':>13} {'feasible c':>14}")
    for t in S.s7c_term_limit_tolerance((9, 10, 11, 12, 15, 18, 27)):
        print(f"{t.term_limit:>4} {t.seats_vacated_per_boundary:>23.2f} {t.minimum_pool:>13} "
              f"{str(t.feasible_c):>14}")
    print()
    print("This is the number that says how much the network tolerates. [DEBT-010] makes")
    print("the term limit a ratchet that can be pushed and not pulled, so once a quorum")
    print("touching two thirds raises T it is raised for ever, and this ceiling is the")
    print("only remaining brake on rotation speed. It must be as tight as the network")
    print("can live with, and the table prices 'live with'.")
    print()
    print(f"  At T = {p.T} the network must refill {p.V / p.T:.2f} seats per boundary. Raising T to the")
    print(f"  recommended ceiling of {R.BOUNDS.validator_max_consecutive_terms_max} lowers that to "
          f"{p.V / R.BOUNDS.validator_max_consecutive_terms_max:.2f}, a "
          f"{100 * (1 - p.T / R.BOUNDS.validator_max_consecutive_terms_max):.0f} % relief valve on")
    print("  candidate supply, bought at the price of a third slower forced rotation.")
    print("  A ceiling of 9 leaves no valve at all and the limit can never be lowered")
    print("  again, so a network that finds its pool too thin would have no lawful move")
    print("  left. A ceiling of 27 buys 67 % relief and makes full forced rotation take")
    print("  half a year at a seven-day boundary, which is most of what the term limit")
    print(f"  was for. {R.BOUNDS.validator_max_consecutive_terms_max} is the tightest value that still leaves a usable valve, and")
    print("  it is reachable only through two signed documents at 5/4 each, spaced by a")
    print("  full election epoch: a process somebody can watch, not an event.")
    print()
    print("  Verified above under GATE-CONSTRAINTS: with T pushed to the ceiling the")
    print("  whole combination still satisfies the constraint block, so the ratchet")
    print("  cannot walk the network into a state where no valid document exists.")

    print()
    print("Coupling 3 — alpha and the shape of the fund — is the subject of the curve")
    print("and the fund-shape sections above. The three are reported together because")
    print("they interact: the eligibility threshold decides who may hold a seat, alpha")
    print("decides what a node that will never hold one earns for being present, and a")
    print("threshold high enough to protect consensus is exactly the threshold that")
    print("makes existence income the only income most devices will ever see.")


# --------------------------------------------------------------------------
# SEC-REQ-16, values, product copy
# --------------------------------------------------------------------------


def section_governance_reach() -> None:
    rule("What governance may still do to these values — and what it may never undo")
    print("A rate limit of 5/4 applied to small integers is not a limit, it is a freeze:")
    print("the reachable interval of a lawful next document is")
    print("[ceil(x*4/5), floor(x*5/4)], and for small x that collapses to a single")
    print("point. Three of the recommended values are therefore chosen ONCE, for ever,")
    print("and the report has to say so (REVIEW-011 RF-006).")
    print()
    for i in S.s9_legal_intervals():
        print("  " + i.line())
    print()
    print("Consequences that follow and are not otherwise visible:")
    print()
    v_max = S.s9b_max_reachable_v()
    print(f"  * V <= {v_max} FOR EVER. `ceil(V/T) <= c` with c frozen at 3 and T capped at")
    print(f"    {R.BOUNDS.validator_max_consecutive_terms_max} by the genesis bounds gives V <= c * T_max = {v_max}. So")
    print(f"    validator_max_set_size = {R.CONSENSUS.validator_max_set_size} and validator_max_set_size_max = "
          f"{R.BOUNDS.validator_max_set_size_max} are")
    print("    HEADROOM THAT CANNOT BE OCCUPIED. The values are harmless but the words")
    print("    around them were wrong: 45 was justified as 'room to grow' and 81 as 3V,")
    print("    and neither is reachable. The Lead may wish to bring")
    print(f"    validator_max_set_size down to {v_max}, which is a value change and not this")
    print("    spec's to make.")
    print()
    print("  * THE COOLDOWN HAS NO SECOND ATTEMPT. The drought table shows a cooldown of")
    print("    1 holding all 27 seats at a standing pool of 33 where 2 settles at 24. If")
    print("    the network discovers that pool in production, the corrective move does")
    print("    not exist: the cooldown cannot be lowered by any lawful document. 2 is")
    print("    still the right choice — it is the compromise between the censor's lever")
    print("    and the term limit's bite — but it is chosen knowing it is final.")
    print()
    print("  * THE SAME IS TRUE OF c AND m, and this study's own reasoning missed it.")
    print("    The argument for T_max = 12 is that a ceiling of 9 would leave a network")
    print("    with a thin pool no lawful move at all. That argument is right about T.")
    print("    For c and for the cooldown THE MOVE DOES NOT EXIST IN ANY CASE: raising c")
    print("    to fill more seats per boundary, or lowering the cooldown, are both")
    print("    rejected on acceptance. The only residual levers are raising T — an")
    print("    irreversible ratchet with two steps left — or stopping.")
    print()
    print("  * min_set/V IS NOT PRESERVED. Measured path, each step inside 5/4 and the")
    print("    monotonic term rule, each accepted by the constraint block:")
    for e in S.s10_min_set_ratio_erosion():
        print(f"      document {e.document}: V={e.V:>2} T={e.T:>2} min_set={e.min_set} "
              f"-> min_set/V = {e.min_set_over_V:.3f}  "
              f"({'accepted' if e.constraint_block_passes else 'REJECTED'})")
    print("    Two documents, one election epoch apart — about fourteen days — take the")
    print("    attrition-capture threshold from two thirds to one half. See the AT-10")
    print("    section for the censorship measurement at the eroded ratio.")
    print()
    print("None of this changes a recommended value. It changes what may honestly be")
    print("said about them: whoever tunes c, m and the cooldown is choosing them once.")


def section_secreq16() -> None:
    rule("SEC-REQ-16 — the three quantities the simulator report must expose")
    print("(a) alpha, the fraction of emission flowing through the availability/existence")
    print("    channel")
    print(f"    recommended target      : {R.ALPHA_TARGET}")
    print(f"    surveillance band       : [{R.ALPHA_SURVEILLANCE_BAND[0]}, "
          f"{R.ALPHA_SURVEILLANCE_BAND[1]}]")
    print(f"    declared tolerance X    : {100 * R.X_DECLARED:.0f} %, equal to the band's upper edge,")
    print("                              a hard ceiling on the whole channel and therefore")
    print("                              provable for every N and H — ABOVE THE DECLARED")
    print("                              USAGE FLOOR. Below it, alpha tends to 1 by")
    print("                              construction and X is not an assertion; the")
    print("                              published quantity there is the absolute amount")
    print("                              diverted per epoch. Nothing in any rule holds")
    print("                              alpha under 0.20 either: see the fund-shape")
    print("                              section on values versus rules.")
    print()
    print("(b) E_p against S(1-k), the reputation-purchase margin of threat-model.md 6.3")
    print()
    print(f"    {'subscription price':>20} {'net cost/fake sub':>19} "
          f"{'margin (fake subs)':>20} {'sustainable':>12}")
    for price in (300_000, 3_000_000, 30_000_000, 60_000_000, 100_000_000):
        m = S.s8_reputation_margin(price)
        print(f"    {price / MICROTOKENS_PER_CREDIT:>17,.1f} cr "
              f"{float(m.net_cost_per_fake_subscriber) / MICROTOKENS_PER_CREDIT:>16,.1f} cr "
              f"{float(m.margin):>20,.1f} {str(m.sustainable):>12}")
    print()
    print("    At the calibrated values the margin is about 3 fake subscribers per")
    print("    controlled node per 30-day period against a 30 cr subscription — not the")
    print("    50x the threat model estimated from the documents' illustrative figures,")
    print("    because those figures were never calibrated against one another. It is")
    print("    still an attack: a 10 000-node fleet funds of order 30 000 fake")
    print("    subscriptions per period, and reputation is what it buys.")
    print()
    print("    The margin is not closable by tuning. Pricing a subscription above a")
    print("    node's existence income is threat-model.md 6.3 option 1, which that")
    print("    document names as the wrong answer because it prices the honest")
    print("    single-device user out of the product. Lowering the creator-share cap k")
    print("    moves the margin by a factor of at most two. The answers are options 2")
    print("    and 3 — weighting subscribers by demonstrated contribution, and not")
    print("    exposing active_subscriber_count in discovery — both consensus and")
    print("    catalogue work under [ADR-006], both excluded from this spec.")
    print("    Reported, not closed.")
    print()
    print("(c) the share of emission capturable by N emulated identities")
    print()
    print(f"    {'N':>8} {'H':>8} {'share at alpha=0.15':>21} {'ceiling X':>11}")
    for N, H in ((10_000, 100), (10_000, 1_000), (10_000, 10_000), (100_000, 10_000)):
        print(f"    {N:>8,} {H:>8,} {100 * R.ALPHA_TARGET * N / (N + H):>20.3f}% "
              f"{100 * R.X_DECLARED:>10.0f}%")
    print()
    print("    Every entry is strictly below alpha, and alpha is held below the band's")
    print("    upper edge, so X bounds the column by construction and not by luck —")
    print("    above the usage floor, and by practice rather than by rule.")
    print()
    print("(d) THE QUANTITY FACING THE USER, added because (a), (b) and (c) all face the")
    print("    attacker or the system and none of them is about the person the network")
    print("    is for (REVIEW-011 RF-005): the fraction of its income an honest")
    print("    availability-only node KEEPS under the AT-07 bench.")
    print()
    res = S.s4_at07()
    keep = 100.0 * res.honest_income_with_fleet / res.honest_income_without_fleet
    print(f"    {'N':>8} {'H':>8} {'honest node keeps':>19}")
    for N, H in ((10_000, 100), (10_000, 1_000), (10_000, 10_000), (10_000, 100_000)):
        print(f"    {N:>8,} {H:>8,} {100.0 * H / (N + H):>18.2f}%")
    print()
    print(f"    At the bench the project declares it tolerates it is {keep:.2f} %. The factor")
    print("    is H/(N+H) and contains no alpha, so no choice of alpha improves it. This")
    print("    is the number that has to be published beside X, because X alone invites")
    print("    the reader to conclude their income is protected to within an order of")
    print("    magnitude when it is not protected at all.")


def section_values() -> None:
    rule("The parameter values")
    p, r, b = R.CONSENSUS, R.REWARD, R.BOUNDS
    print(f"Assumed block interval: {R.BLOCK_INTERVAL_SECONDS} s. The block-count parameters are")
    print("chosen against it; a different block interval rescales them and the constraint")
    print("block must be re-run. The assumption is declared because no protocol document")
    print("fixes a block interval.")
    print()
    print("consensus_parameters (election subset)")
    for name, value, note in (
        ("election_epoch_blocks", p.election_epoch_blocks, "7 days"),
        ("candidacy_close_blocks", p.candidacy_close_blocks, "1 day before the boundary"),
        ("election_entropy_blocks", p.election_entropy_blocks, "1 hour"),
        ("validator_min_set_size", p.validator_min_set_size,
         "2V/3 AT THIS V: closes attrition below two thirds; the ratio is preserved by no rule"),
        ("validator_target_set_size", p.validator_target_set_size, "V"),
        ("validator_max_set_size", p.validator_max_set_size,
         "nominal headroom; V is permanently capped at 36 by the frozen c"),
        ("validator_churn_cap_seats", p.validator_churn_cap_seats,
         "c = V/T exactly; at m = 3 no slack. FROZEN by the 5/4 rate limit"),
        ("validator_max_consecutive_terms", p.validator_max_consecutive_terms,
         "T = 3m, the smallest value the horizon admits"),
        ("validator_cooldown_epochs", p.validator_cooldown_epochs,
         "short: multiplies a censor's lever and drains the pool. FROZEN: no lawful document may change it"),
        ("validator_min_capture_epochs", p.validator_min_capture_epochs,
         "m = 3, the attrition horizon; more would be self-deception. FROZEN"),
    ):
        print(f"  {name:<34} {value:>10,}   # {note}")
    print()
    print("reward_policy (eligibility and emission subset)")
    for name, value, note in (
        ("reward_epoch_ms", r.reward_epoch_ms, "1 day"),
        ("existence_fund_microtokens_per_epoch", r.existence_fund_microtokens_per_epoch,
         "alpha = 0.15 at reference usage; governed with NO ceiling and NO rate limit"),
        ("availability_microtokens_per_unit", r.availability_microtokens_per_unit,
         "MUST be 0 (AT-07 counter-example) — a VALUE, imposed by no rule"),
        ("storage_units_per_contribution_unit", r.storage_units_per_contribution_unit,
         "1 unit per GiB-epoch proven"),
        ("compute_units_per_contribution_unit", r.compute_units_per_contribution_unit,
         "1 unit per million fuel re-executed"),
        ("validator_eligibility_threshold_units", r.validator_eligibility_threshold_units,
         "about 18 GiB sustained across the window"),
        ("validator_eligibility_window_epochs", r.validator_eligibility_window_epochs, "4 weeks"),
        ("validator_eligibility_min_issuers", r.validator_eligibility_min_issuers,
         "the price of a fabricated candidate is linear in this"),
        ("publisher_reward_cap_numerator", r.publisher_reward_cap_numerator, "k = 1/2"),
        ("publisher_reward_cap_denominator", r.publisher_reward_cap_denominator, ""),
    ):
        print(f"  {name:<38} {value:>16,}   # {note}")
    print()
    print("ElectionBounds (genesis trust anchor, outside on-chain governance)")
    for name, value, note in (
        ("election_epoch_blocks_max", b.election_epoch_blocks_max, "at most a doubling"),
        ("validator_max_consecutive_terms_max", b.validator_max_consecutive_terms_max,
         "the [DEBT-010] residual guard: tightest value leaving a usable valve"),
        ("validator_max_set_size_max", b.validator_max_set_size_max,
         "nominally 3V; unreachable, see the governance-reach section"),
        ("validator_min_set_size_min", b.validator_min_set_size_min,
         "pinned at the chosen minimum: it may never be lowered"),
        ("validator_min_capture_epochs_min", b.validator_min_capture_epochs_min,
         "pinned at the attrition horizon"),
        ("election_parameter_change_numerator", b.election_parameter_change_numerator,
         "5/4 per document"),
        ("election_parameter_change_denominator", b.election_parameter_change_denominator, ""),
        ("election_parameter_min_activation_gap_blocks",
         b.election_parameter_min_activation_gap_blocks, "one full election epoch"),
    ):
        print(f"  {name:<45} {value:>10,}   # {note}")
    print()
    print("Twenty-two values, plus alpha, its band, X, and the fund cap. Every one of")
    print("them is checked against the constraint block above, not asserted to pass it.")
    print()
    print("For which of them a lawful next document may still move, and which three are")
    print("frozen for ever by the 5/4 rate limit, see the governance-reach section: c, m")
    print("and the cooldown are chosen once, and V is capped at 36 permanently.")


PRODUCT_COPY = """\
Primary line (dashboard, beside the figure):

    Existence income — your share of this epoch's network fund

Supporting sentence (first run, and the help panel):

    Every epoch the network issues a fixed fund and splits it equally among all
    nodes that proved they were there. Your income is a share of that fund, not
    a fixed amount: it goes down when more nodes are present and up when fewer
    are. The fund is capped, so nobody can make it bigger by adding devices.

The one-line answer to "why did my income drop?":

    The fund did not shrink - it was shared with more nodes this epoch.

The honest note the network owes its users (help panel, not the dashboard):

    Some of the nodes sharing the fund are not real people. The protocol cannot
    tell a phone from a program pretending to be one, and it does not claim to.
    What it does guarantee is that no amount of pretending creates new credits:
    a fake node can only take a slice, never bake a bigger cake.

    But it takes that slice instead of you. Fake nodes do not reduce the fund -
    they share it - so every one of them makes your share smaller, and the
    network cannot stop that. It is the cost of letting anyone join with a
    device they already own, and it is a cost you pay directly.

    Once the network is carrying real work, we publish the share of all issuance
    that flows through this fund every epoch, and we commit to keeping it under
    20 %. Before then that share is close to everything, because there is
    almost nothing else being issued yet, so we publish the amount instead of
    the share - it is the honest number while the network is starting up.

Words to avoid, with the reason:

    "guaranteed"   - it is not; the share moves every epoch.
    "basic income" - imports an expectation of a fixed floor denominated in
                     money, which is the one thing a credit is not.
    "reward"       - this fund pays for presence, not for work; the work
                     channels are named separately and paid per unit.
    "protected"    - the 20 % commitment bounds what a fleet takes of ISSUANCE.
                     It does not bound what an individual user loses, which is
                     set by how many nodes are present. Never let the two be
                     read as the same promise.
    "$", or any glyph before the number - [ADR-009]: the unit is written after
                     the number ("1 240 cr"), because that is the grammar of a
                     measure and not of a currency.
"""


def section_product_copy() -> None:
    rule("Product wording — existence income is a variable share (English, for the UI)")
    print(PRODUCT_COPY)


# --------------------------------------------------------------------------


def main(argv: list[str]) -> int:
    only_gates = len(argv) > 1 and argv[1] == "gates"
    print("Coblox economic simulator — SPEC-007")
    print(f"seed: {S.SEED}")
    print("deterministic: every draw is SHA-256 of (seed, stream, index); no RNG state,")
    print("so the figures are reproducible across Python versions and platforms.")
    g1 = gate_model_validated()
    g2 = gate_constraints()
    if not only_gates:
        section_curve()
        section_dilution()
        section_drift()
        section_fund_shape()
        section_at07()
        section_at10()
        section_couplings()
        section_governance_reach()
        section_secreq16()
        section_values()
        section_product_copy()
    rule("Gate summary")
    print(f"GATE-MODEL-VALIDATED : {'PASS' if g1 else 'FAIL'}")
    print(f"GATE-CONSTRAINTS     : {'PASS' if g2 else 'FAIL'}")
    return 0 if (g1 and g2) else 1


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
