package io.provenance.bilateral.extensions

import cosmos.base.abci.v1beta1.Abci
import cosmos.base.abci.v1beta1.Abci.MsgData
import cosmos.base.v1beta1.CoinOuterClass
import cosmwasm.wasm.v1.Tx.MsgExecuteContractResponse
import io.provenance.bilateral.models.AttributeRequirement
import io.provenance.bilateral.models.BidCollateral
import io.provenance.bilateral.models.BidOrder
import io.provenance.bilateral.models.RequestDescriptor
import io.provenance.bilateral.models.enums.AttributeRequirementType
import io.provenance.bilateral.models.enums.BilateralRequestType
import io.provenance.bilateral.util.ObjectMapperProvider.OBJECT_MAPPER
import io.provenance.scope.util.toByteString
import org.bouncycastle.util.encoders.Hex
import org.junit.jupiter.api.Test
import java.time.OffsetDateTime
import java.util.TimeZone
import java.util.UUID
import kotlin.test.BeforeTest
import kotlin.test.assertEquals

class StringExtensionsTest {
    @BeforeTest
    fun setTimeZone() {
        TimeZone.setDefault(TimeZone.getTimeZone("UTC"))
    }

    @Test
    fun testExecuteContractDataToJsonBytes() {
        val bidOrder = BidOrder(
            id = UUID.randomUUID().toString(),
            bidType = BilateralRequestType.COIN_TRADE,
            owner = "bidder",
            collateral = BidCollateral.CoinTrade(
                base = listOf(CoinOuterClass.Coin.newBuilder().setAmount("100").setDenom("nhash").build()),
                quote = listOf(CoinOuterClass.Coin.newBuilder().setAmount("100").setDenom("nhash").build()),
            ),
            descriptor = RequestDescriptor(
                description = "some description",
                effectiveTime = OffsetDateTime.now(),
                attributeRequirement = AttributeRequirement(
                    attributes = listOf("a.pb", "b.pb", "c.pb"),
                    requirementType = AttributeRequirementType.ALL,
                )
            )
        )
        assertEquals(
            expected = OBJECT_MAPPER.readValue(OBJECT_MAPPER.writeValueAsBytes(bidOrder), BidOrder::class.java),
            actual = bidOrder,
            message = "Sanity check: BidOrder written as bytes can deserialize back to itself",
        )
        val msgData = Abci.TxMsgData.newBuilder().also { txMsgData ->
            txMsgData.addData(
                MsgData.newBuilder()
                    .setMsgType("/cosmos.wasm.v1.MsgExecuteContract")
                    .setData(
                        MsgExecuteContractResponse.newBuilder().also { msgExecuteContract ->
                            msgExecuteContract.data = OBJECT_MAPPER.writeValueAsBytes(bidOrder).toByteString()
                        }.build().toByteString()
                    )
            )
        }.build()
        val hex = Hex.encode(msgData.toByteArray())
        val txResponse = Abci.TxResponse.newBuilder().also { txResponse ->
            txResponse.data = hex.toString(Charsets.UTF_8).uppercase()
        }.build()
        val bidOrderBytes = txResponse.executeContractDataToJsonBytes()
        val bidOrderDeserialized = OBJECT_MAPPER.readValue(bidOrderBytes, BidOrder::class.java)
        assertEquals(
            expected = bidOrder,
            actual = bidOrderDeserialized,
            message = "Expected the bid order to properly make the round trip to TxResponse and back",
        )
    }
}
