package io.provenance.bilateral.contract

import io.provenance.bilateral.execute.Ask.CoinTradeAsk
import io.provenance.bilateral.execute.Bid.CoinTradeBid
import io.provenance.bilateral.execute.CreateAsk
import io.provenance.bilateral.execute.CreateBid
import org.junit.jupiter.api.Test
import testconfiguration.ContractIntTest
import testconfiguration.extensions.clearFees
import testconfiguration.extensions.getBalance
import testconfiguration.extensions.setFees
import testconfiguration.functions.assertSucceeds
import testconfiguration.functions.giveTestDenom
import testconfiguration.functions.newCoin
import testconfiguration.functions.newCoins
import java.util.UUID
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class FeesIntTest : ContractIntTest() {
    @Test
    fun testCreateAskWithFee() {
        val askerDenom = "cointradecreateaskwithfeebase"
        giveTestDenom(
            pbClient = pbClient,
            initialHoldings = newCoin(1000, askerDenom),
            receiverAddress = asker.address(),
        )
        // Fee == 1000 hash
        val askFee = 1_000_000_000_000
        bilateralClient.setFees(askFee = askFee.toBigInteger())
        val startingHashBalance = pbClient.getBalance(asker.address(), "nhash")
        assertTrue(
            actual = startingHashBalance > askFee * 3,
            message = "The asker account does not have enough hash to perform this test :(",
        )
        val quote = newCoins(1000, "somequote")
        val base = newCoins(1000, askerDenom)
        val askUuid = UUID.randomUUID()
        val response = assertSucceeds("Ask should be created without error") {
            createAsk(
                createAsk = CreateAsk(
                    ask = CoinTradeAsk(
                        id = askUuid.toString(),
                        base = base,
                        quote = quote,
                    ),
                ),
            )
        }
        assertEquals(
            expected = "${askFee}nhash",
            actual = response.askFeeCharged,
            message = "The correct ask fee charged value should be omitted",
        )
        val paidNhashForAskCreation = startingHashBalance - pbClient.getBalance(asker.address(), "nhash")
        assertTrue(
            actual = paidNhashForAskCreation >= askFee,
            message = "The ask fee should be paid out during the transaction, but the asker only paid: ${paidNhashForAskCreation}nhash",
        )
        bilateralClient.clearFees()
    }

    @Test
    fun testCreateBidWithFee() {
        val bidderDenom = "cointradecreatebidwithfeebase"
        giveTestDenom(
            pbClient = pbClient,
            initialHoldings = newCoin(1000, bidderDenom),
            receiverAddress = bidder.address(),
        )
        // Fee == 1000 hash
        val bidFee = 1_000_000_000_000
        bilateralClient.setFees(bidFee = bidFee.toBigInteger())
        val startingHashBalance = pbClient.getBalance(bidder.address(), "nhash")
        assertTrue(
            actual = startingHashBalance > bidFee * 3,
            message = "The bidder account does not have enough hash to perform this test :(((",
        )
        val quote = newCoins(1000, bidderDenom)
        val base = newCoins(1000, "somebasedenom")
        val bidUuid = UUID.randomUUID()
        val response = assertSucceeds("Bid should be created without error") {
            createBid(
                createBid = CreateBid(
                    bid = CoinTradeBid(
                        id = bidUuid.toString(),
                        base = base,
                        quote = quote,
                    ),
                ),
            )
        }
        assertEquals(
            expected = "${bidFee}nhash",
            actual = response.bidFeeCharged,
            message = "The correct bid fee charged value should be omitted",
        )
        val paidNhashForBidCreation = startingHashBalance - pbClient.getBalance(bidder.address(), "nhash")
        assertTrue(
            actual = paidNhashForBidCreation >= bidFee,
            message = "The bid fee should be paid out during the transaction, but the bidder only paid: ${paidNhashForBidCreation}nhash",
        )
        bilateralClient.clearFees()
    }
}
