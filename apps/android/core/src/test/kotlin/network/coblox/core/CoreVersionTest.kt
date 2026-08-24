package network.coblox.core

import org.junit.Assert.assertEquals
import org.junit.Test

class CoreVersionTest {
  @Test
  fun `generated UniFFI binding returns the core version`() {
    assertEquals("0.1.0", coreVersion())
  }
}
