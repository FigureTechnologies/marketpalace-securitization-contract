package io.provenance.bilateral.contract

import io.provenance.bilateral.execute.UpdateSettings
import org.junit.jupiter.api.Test
import testconfiguration.ContractIntTest
import testconfiguration.accounts.BilateralAccounts
import testconfiguration.functions.assertSucceeds
import java.math.BigInteger
import kotlin.test.assertEquals
import kotlin.test.assertFails

class UpdateSettingsIntTest : ContractIntTest() {
    @Test
    fun testUpdateSettingAll() {
        val firstUpdateSettings = UpdateSettings(
            newAdminAddress = BilateralAccounts.askerAccount.address(),
            newCreateAskNhashFee = 100.toBigInteger(),
            newCreateBidNhashFee = 150.toBigInteger(),
        )
        assertFails("An update not originating from the admin should be rejected") {
            bilateralClient.updateSettings(firstUpdateSettings, BilateralAccounts.askerAccount)
        }
        val firstResponse = assertSucceeds("A valid update sent from the admin should succeed") {
            bilateralClient.updateSettings(firstUpdateSettings, BilateralAccounts.adminAccount)
        }
        assertEquals(
            expected = BilateralAccounts.askerAccount.address(),
            actual = firstResponse.newAdminAddress,
            message = "The new admin address should be in the response",
        )
        assertEquals(
            expected = "100nhash",
            actual = firstResponse.newAskFee,
            message = "The new ask fee should be in the response",
        )
        assertEquals(
            expected = "150nhash",
            actual = firstResponse.newBidFee,
            message = "The new bid fee should be in the response",
        )
        assertContractInfoValuesWereChanged(firstUpdateSettings)
        // Put everything back to normal
        val secondUpdateSettings = UpdateSettings(
            newAdminAddress = BilateralAccounts.adminAccount.address(),
            newCreateAskNhashFee = BigInteger.ZERO,
            newCreateBidNhashFee = BigInteger.ZERO,
        )
        assertFails("An update not originating from the asker should be rejected") {
            bilateralClient.updateSettings(secondUpdateSettings, BilateralAccounts.adminAccount)
        }
        val secondResponse = assertSucceeds("A valid update sent from the asker should succeed") {
            bilateralClient.updateSettings(secondUpdateSettings, BilateralAccounts.askerAccount)
        }
        assertEquals(
            expected = BilateralAccounts.adminAccount.address(),
            actual = secondResponse.newAdminAddress,
            message = "The new new admin address should be in the response",
        )
        assertEquals(
            expected = "disabled",
            actual = secondResponse.newAskFee,
            message = "The new ask fee should be disabled, indicating that it was cleared",
        )
        assertEquals(
            expected = "disabled",
            actual = secondResponse.newBidFee,
            message = "The new bid fee should be disabled, indicating that it was cleared",
        )
        assertContractInfoValuesWereChanged(secondUpdateSettings)
    }

    private fun assertContractInfoValuesWereChanged(updateSettings: UpdateSettings) {
        val contractInfo = bilateralClient.getContractInfo()
        assertEquals(
            expected = updateSettings.update.newAdminAddress,
            actual = contractInfo.admin,
            message = "The admin address should be properly altered to match the request",
        )
        assertEquals(
            expected = updateSettings.update.newCreateAskNhashFee,
            actual = contractInfo.createAskNhashFee,
            message = "The ask fee should be properly altered to match the request",
        )
        assertEquals(
            expected = updateSettings.update.newCreateBidNhashFee,
            actual = contractInfo.createBidNhashFee,
            message = "The bid fee should be properly altered to match the request",
        )
    }
}
