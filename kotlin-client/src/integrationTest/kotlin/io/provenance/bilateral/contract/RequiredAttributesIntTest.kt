package io.provenance.bilateral.contract

import cosmos.base.v1beta1.CoinOuterClass.Coin
import cosmos.tx.v1beta1.ServiceOuterClass
import io.provenance.attribute.v1.AttributeType
import io.provenance.attribute.v1.MsgAddAttributeRequest
import io.provenance.bilateral.execute.Ask.CoinTradeAsk
import io.provenance.bilateral.execute.Bid.CoinTradeBid
import io.provenance.bilateral.execute.CreateAsk
import io.provenance.bilateral.execute.CreateBid
import io.provenance.bilateral.execute.ExecuteMatch
import io.provenance.bilateral.models.AttributeRequirement
import io.provenance.bilateral.models.RequestDescriptor
import io.provenance.bilateral.models.enums.AttributeRequirementType
import io.provenance.client.grpc.BaseReqSigner
import io.provenance.client.grpc.Signer
import io.provenance.client.protobuf.extensions.toAny
import io.provenance.client.protobuf.extensions.toTxBody
import io.provenance.scope.util.toByteString
import mu.KLogging
import org.junit.jupiter.api.Test
import testconfiguration.ContractIntTest
import testconfiguration.extensions.checkIsSuccess
import testconfiguration.functions.assertAskExists
import testconfiguration.functions.assertBidExists
import testconfiguration.functions.assertSucceeds
import testconfiguration.functions.bindNamesToSigner
import testconfiguration.functions.newCoins
import java.time.OffsetDateTime
import java.util.UUID
import kotlin.test.assertEquals
import kotlin.test.assertFails
import kotlin.test.assertTrue

class RequiredAttributesIntTest : ContractIntTest() {
    private companion object : KLogging() {
        private val DEFAULT_QUOTE: List<Coin> = newCoins(100, "nhash")
        private val DEFAULT_BASE: List<Coin> = newCoins(100, "nhash")
    }

    @Test
    fun testRequiredAttributesTypeAll() {
        val attributePrefix = "testRequiredAttributesTypeAll".lowercase()
        val testAttributes = listOf("${attributePrefix}a.pb", "${attributePrefix}b.pb", "${attributePrefix}c.pb")
        bindNamesToSigner(
            pbClient = pbClient,
            names = testAttributes,
            signer = admin,
            restricted = true,
        )
        val askUuid = UUID.randomUUID()
        logger.info("Creating an ask with UUID: $askUuid, requiring all attributes: $testAttributes")
        createAndSendAsk(askUuid, testAttributes, AttributeRequirementType.ALL)
        val bidUuid = UUID.randomUUID()
        logger.info("Creating bid with UUID: $bidUuid, requiring all attributes: $testAttributes")
        createAndSendBid(bidUuid, testAttributes, AttributeRequirementType.ALL)
        val executeMatch = ExecuteMatch(
            askId = askUuid.toString(),
            bidId = bidUuid.toString(),
        )
        logger.info("Executing match for ask [$askUuid] and bid [$bidUuid] and expecting a failure")
        assertFails("Expected the match to fail because the asker and bidder are both missing the attributes") {
            executeMatch(executeMatch)
        }
        addDummyAttributesToAddress(
            attributeOwner = admin,
            attributes = testAttributes,
            targetAddress = asker.address(),
        )
        logger.info("Executing match for ask [$askUuid] and bid [$bidUuid] and expecting a failure")
        assertFails("Expected the match to fail because the bidder is still missing the attributes") {
            executeMatch(executeMatch, admin)
        }
        addDummyAttributesToAddress(
            attributeOwner = admin,
            attributes = testAttributes,
            targetAddress = bidder.address(),
        )
        logger.info("Executing match for ask [$askUuid] and bid [$bidUuid] and expecting success")
        val matchResponse = assertSucceeds("Expected match to succeed now that all required attributes are held by ask and bid") {
            executeMatch(executeMatch, admin)
        }
        assertTrue(matchResponse.askDeleted, "The ask should be deleted")
        assertTrue(matchResponse.bidDeleted, "The bid should be deleted")
    }

    @Test
    fun testRequiredAttributesTypeAny() {
        val attributePrefix = "testRequiredAttributesTypeAny".lowercase()
        val testAttributes = listOf("${attributePrefix}a.pb", "${attributePrefix}b.pb", "${attributePrefix}c.pb")
        bindNamesToSigner(
            pbClient = pbClient,
            names = testAttributes,
            signer = admin,
            restricted = true,
        )
        val askUuid = UUID.randomUUID()
        logger.info("Creating an ask with UUID: $askUuid, requiring any of attributes: $testAttributes")
        createAndSendAsk(askUuid, testAttributes, AttributeRequirementType.ANY)
        val bidUuid = UUID.randomUUID()
        logger.info("Creating bid with UUID: $bidUuid, requiring any of attributes: $testAttributes")
        createAndSendBid(bidUuid, testAttributes, AttributeRequirementType.ANY)
        val executeMatch = ExecuteMatch(
            askId = askUuid.toString(),
            bidId = bidUuid.toString(),
        )
        logger.info("Executing match for ask [$askUuid] and bid [$bidUuid] and expecting a failure")
        assertFails("Expected the match to fail because the asker and bidder are both missing the attributes") {
            executeMatch(executeMatch, admin)
        }
        // Only add a random one of the attributes to the asker account to spice things up and verify that only one of
        // any of the values is required
        addDummyAttributesToAddress(
            attributeOwner = admin,
            attributes = testAttributes.random().let(::listOf),
            targetAddress = asker.address(),
        )
        logger.info("Executing match for ask [$askUuid] and bid [$bidUuid] and expecting a failure")
        assertFails("Expected the match to fail because the bidder is still missing attributes") {
            executeMatch(executeMatch, admin)
        }
        // Only add a random one of the attributes to the bidder account to spice things up and verify that only one of
        // any of the values is required
        addDummyAttributesToAddress(
            attributeOwner = admin,
            attributes = testAttributes.random().let(::listOf),
            targetAddress = bidder.address(),
        )
        logger.info("Executing match for ask [$askUuid] and bid [$bidUuid] and expecting success")
        val matchResponse = assertSucceeds("Expected the match to succeed now that both ask and bid have one attribute") {
            executeMatch(executeMatch, admin)
        }
        assertTrue(matchResponse.askDeleted, "The ask should be deleted")
        assertTrue(matchResponse.bidDeleted, "The bid should be deleted")
    }

    @Test
    fun testRequiredAttributesTypeNone() {
        val attributePrefix = "testRequiredAttributesTypeNone".lowercase()
        val testAttributes = listOf("${attributePrefix}a.pb", "${attributePrefix}b.pb", "${attributePrefix}c.pb")
        bindNamesToSigner(
            pbClient = pbClient,
            names = testAttributes,
            signer = admin,
            restricted = true,
        )
        val firstAskUuid = UUID.randomUUID()
        logger.info("Creating an ask with UUID: $firstAskUuid, requiring none of attributes: $testAttributes")
        createAndSendAsk(firstAskUuid, testAttributes, AttributeRequirementType.NONE)
        val firstBidUuid = UUID.randomUUID()
        logger.info("Creating bid with UUID: $firstBidUuid, requiring none of attributes: $testAttributes")
        createAndSendBid(firstBidUuid, testAttributes, AttributeRequirementType.NONE)
        val firstExecuteMatch = ExecuteMatch(
            askId = firstAskUuid.toString(),
            bidId = firstBidUuid.toString(),
        )
        logger.info("Executing match for ask [$firstAskUuid] and bid [$firstBidUuid]")
        val matchResponse = assertSucceeds("Expecting the match to succeed because neither account has any of the specified attributes") {
            executeMatch(firstExecuteMatch, admin)
        }
        assertTrue(matchResponse.askDeleted, "The ask should be deleted")
        assertTrue(matchResponse.bidDeleted, "The bid should be deleted")
        val secondAskUuid = UUID.randomUUID()
        logger.info("Creating ask with uuid: $secondAskUuid, requiring none of attributes: $testAttributes")
        createAndSendAsk(secondAskUuid, testAttributes, AttributeRequirementType.NONE)
        val secondBidUuid = UUID.randomUUID()
        logger.info("Creating bid with uuid: $secondBidUuid, requiring none of attributes: $testAttributes")
        createAndSendBid(secondBidUuid, testAttributes, AttributeRequirementType.NONE)
        val secondExecuteMatch = ExecuteMatch(
            askId = secondAskUuid.toString(),
            bidId = secondBidUuid.toString(),
        )
        // Only add a random one of the attributes to the asker account to spice things up and verify that only one of
        // any of the values is required to cause a rejection
        addDummyAttributesToAddress(
            attributeOwner = admin,
            attributes = testAttributes.random().let(::listOf),
            targetAddress = asker.address(),
        )
        logger.info("Executing match for ask [$secondAskUuid] and bid [$secondBidUuid] and expecting a failure")
        assertFails("Expected the match to fail because the asker has one of the attributes") {
            executeMatch(secondExecuteMatch, admin)
        }
        // Only add a random one of the attributes to the bidder account to spice things up and verify that only one of
        // any of the values is required to cause a rejection
        addDummyAttributesToAddress(
            attributeOwner = admin,
            attributes = testAttributes.random().let(::listOf),
            targetAddress = bidder.address(),
        )
        logger.info("Executing match for ask [$secondAskUuid] and bid [$secondBidUuid] and expecting a failure")
        assertFails("Expected the match to fail because both asker and bidder have attributes that are not allowed") {
            executeMatch(secondExecuteMatch, admin)
        }
        bilateralClient.assertAskExists(secondAskUuid.toString(), "Expected the ask to exist because a match was never made")
        bilateralClient.assertBidExists(secondBidUuid.toString(), "Expected the bid to exist because a match was never made")
    }

    private fun addDummyAttributesToAddress(
        attributeOwner: Signer,
        attributes: List<String>,
        targetAddress: String,
    ) {
        attributes.map { attributeName ->
            MsgAddAttributeRequest.newBuilder().also { addAttribute ->
                addAttribute.account = targetAddress
                addAttribute.attributeType = AttributeType.ATTRIBUTE_TYPE_STRING
                addAttribute.name = attributeName
                addAttribute.owner = attributeOwner.address()
                addAttribute.value = "dummyvalue".toByteString()
            }.build().toAny()
        }.also { attributeMsgs ->
            logger.info("Adding attributes $attributes to address [$targetAddress] with owner address [${attributeOwner.address()}]")
            pbClient.estimateAndBroadcastTx(
                txBody = attributeMsgs.toTxBody(),
                signers = listOf(BaseReqSigner(attributeOwner)),
                mode = ServiceOuterClass.BroadcastMode.BROADCAST_MODE_BLOCK,
                gasAdjustment = 1.3,
            ).checkIsSuccess()
        }
    }

    private fun createAndSendAsk(
        askUuid: UUID,
        attributes: List<String>,
        requirementType: AttributeRequirementType,
    ) {
        val createAsk = CreateAsk(
            ask = CoinTradeAsk(
                id = askUuid.toString(),
                quote = DEFAULT_QUOTE,
                base = DEFAULT_BASE,
            ),
            descriptor = RequestDescriptor(
                description = "Example description",
                effectiveTime = OffsetDateTime.now(),
                attributeRequirement = AttributeRequirement(attributes, requirementType),
            ),
        )
        val createResponse = createAsk(createAsk)
        assertEquals(
            expected = createAsk.descriptor?.effectiveTime,
            actual = createResponse.askOrder.descriptor?.effectiveTime,
            message = "Expected the effective time to correctly deserialize from the contract response",
        )
    }

    private fun createAndSendBid(
        bidUuid: UUID,
        attributes: List<String>,
        requirementType: AttributeRequirementType,
    ) {
        val createBid = CreateBid(
            bid = CoinTradeBid(
                id = bidUuid.toString(),
                base = DEFAULT_BASE,
                quote = DEFAULT_QUOTE,
            ),
            descriptor = RequestDescriptor(
                description = "Example description",
                effectiveTime = OffsetDateTime.now(),
                attributeRequirement = AttributeRequirement(attributes, requirementType),
            ),
        )
        val createResponse = createBid(createBid)
        assertEquals(
            expected = createBid.descriptor?.effectiveTime,
            actual = createResponse.bidOrder.descriptor?.effectiveTime,
            message = "Expected the effective time to correctly deserialize from the contract response",
        )
    }
}
