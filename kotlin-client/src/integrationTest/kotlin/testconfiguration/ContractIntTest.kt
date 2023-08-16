package testconfiguration

import io.provenance.bilateral.client.BilateralContractClient
import io.provenance.bilateral.client.BilateralContractClientLogger
import io.provenance.bilateral.execute.CreateAsk
import io.provenance.bilateral.execute.CreateBid
import io.provenance.bilateral.execute.ExecuteMatch
import io.provenance.bilateral.execute.UpdateAsk
import io.provenance.bilateral.execute.UpdateBid
import io.provenance.bilateral.models.ContractInfo
import io.provenance.bilateral.models.executeresponse.CancelAskResponse
import io.provenance.bilateral.models.executeresponse.CancelBidResponse
import io.provenance.bilateral.models.executeresponse.CreateAskResponse
import io.provenance.bilateral.models.executeresponse.CreateBidResponse
import io.provenance.bilateral.models.executeresponse.ExecuteMatchResponse
import io.provenance.bilateral.models.executeresponse.UpdateAskResponse
import io.provenance.bilateral.models.executeresponse.UpdateBidResponse
import io.provenance.bilateral.util.ContractAddressResolver
import io.provenance.client.grpc.GasEstimationMethod
import io.provenance.client.grpc.PbClient
import io.provenance.client.grpc.Signer
import mu.KLogging
import org.testcontainers.containers.BindMode
import testconfiguration.accounts.BilateralAccounts.adminAccount
import testconfiguration.accounts.BilateralAccounts.askerAccount
import testconfiguration.accounts.BilateralAccounts.bidderAccount
import testconfiguration.accounts.BilateralAccounts.fundingAccount
import testconfiguration.functions.assertAskExists
import testconfiguration.functions.assertAskIsDeleted
import testconfiguration.functions.assertBidExists
import testconfiguration.functions.assertBidIsDeleted
import testconfiguration.testcontainers.ProvenanceTestContainer
import testconfiguration.testcontainers.ProvenanceWaitStrategy
import testconfiguration.util.BilateralSmartContractUtil
import testconfiguration.util.CoinFundingUtil
import testconfiguration.util.ContractInstantiationResult
import testconfiguration.util.ScopeWriteUtil
import java.net.URI
import java.util.TimeZone
import kotlin.system.exitProcess
import kotlin.test.AfterTest
import kotlin.test.BeforeTest
import kotlin.test.assertEquals

abstract class ContractIntTest {
    companion object : KLogging() {
        private const val CLEANUP_PREFIX: String = "[TESTCLEANUP]:"

        private var containerIsStarted: Boolean = false

        private val container: ProvenanceTestContainer = ProvenanceTestContainer()
            .withNetworkAliases("provenance")
            .withClasspathResourceMapping("data/provenance", "/home/provenance_seed", BindMode.READ_ONLY)
            .withExposedPorts(1317, 9090, 26657)
            .withCommand("bash", "-c", "cp -rn /home/provenance_seed/* /home/provenance && /usr/bin/provenanced -t --home /home/provenance start")
            .waitingFor(ProvenanceWaitStrategy(fundingAccount.address()))

        val pbClient: PbClient by lazy {
            PbClient(
                chainId = "chain-local",
                channelUri = URI.create("http://${container.host}:${container.getMappedPort(9090)}"),
                gasEstimationMethod = GasEstimationMethod.MSG_FEE_CALCULATION,
            )
        }

        lateinit var contractInfo: ContractInstantiationResult

        val bilateralClient by lazy {
            BilateralContractClient.builder(
                pbClient = pbClient,
                addressResolver = ContractAddressResolver.ProvidedAddress(contractInfo.contractAddress)
            ).setLogger(
                BilateralContractClientLogger.Custom(
                    infoLogger = { message -> logger.info(message) },
                    errorLogger = { message, e -> logger.error(message, e) },
                )
            ).build()
        }
    }

    private lateinit var contractInternalInfoSnapshot: ContractInfo
    private var createdAsks = mutableMapOf<String, CreatedAsk>()
    private var createdBids = mutableMapOf<String, CreatedBid>()

    init {
        if (!containerIsStarted) {
            try {
                container.start()
            } catch (e: Exception) {
                logger.error("Failed to start provenance container", e)
                exitProcess(1)
            }
            try {
                logger.info("Normalizing timezone to UTC to ensure deserialized values match in tests")
                TimeZone.setDefault(TimeZone.getTimeZone("UTC"))
                CoinFundingUtil.fundAccounts(
                    pbClient = pbClient,
                    senderAccount = fundingAccount,
                    receiverAccounts = listOf(adminAccount, askerAccount, bidderAccount),
                )
                logger.info("Setting up the local bilateral exchange smart contract")
                contractInfo = BilateralSmartContractUtil.instantiateSmartContract(pbClient, adminAccount)
                logger.info("Successfully established the contract with name [${contractInfo.contractBindingName}] at address [${contractInfo.contractAddress}]")
                ScopeWriteUtil.writeInitialScopeSpecs(pbClient)
            } catch (e: Exception) {
                logger.error("Failed to fund accounts and/or stand up the exchange smart contract", e)
                exitProcess(1)
            }
            containerIsStarted = true
        }
    }

    // Expose these values for ease of use and syntax cleanup
    val asker = askerAccount
    val bidder = bidderAccount
    val admin = adminAccount

    @BeforeTest
    fun beforeTest() {
        if (createdAsks.isNotEmpty()) {
            logger.info("$CLEANUP_PREFIX Cleaning up ask data: ${entryCountDisplay(createdAsks)}")
            createdAsks.clear()
        }
        var tokens = mutableListOf<Char>()
        if (createdBids.isNotEmpty()) {
            logger.info("$CLEANUP_PREFIX Cleaning up bid data: ${entryCountDisplay(createdBids)}")
            createdBids.clear()
        }
    }

    @AfterTest
    fun afterTest() {
        createdAsks.values.forEach { createdAsk ->
            logger.info("$CLEANUP_PREFIX Cancelling ask [${createdAsk.askId}]")
            bilateralClient.cancelAsk(askId = createdAsk.askId, signer = createdAsk.signer)
            logger.info("$CLEANUP_PREFIX Cancelled ask [${createdAsk.askId}]")
        }
        createdBids.values.forEach { createdBid ->
            logger.info("$CLEANUP_PREFIX Cancelling bid [${createdBid.bidId}]")
            bilateralClient.cancelBid(bidId = createdBid.bidId, signer = createdBid.signer)
            logger.info("$CLEANUP_PREFIX Cancelled bid [${createdBid.bidId}]")
        }
    }

    fun createAsk(
        createAsk: CreateAsk,
        signer: Signer = asker,
    ): CreateAskResponse = bilateralClient.createAsk(
        createAsk = createAsk,
        signer = signer,
    ).also { createResponse ->
        val askId = createAsk.ask.mapToId()
        assertEquals(
            expected = askId,
            actual = createResponse.askId,
            message = "The correct ask id was not returned in the create ask response",
        )
        assertEquals(
            expected = bilateralClient.assertAskExists(askId),
            actual = createResponse.askOrder,
            message = "The ask order stored in the contract does not match the value returned in the create ask response",
        )
        createdAsks += CreatedAsk(askId, signer).let { askId to it }
    }

    fun updateAsk(
        updateAsk: UpdateAsk,
        signer: Signer = asker,
    ): UpdateAskResponse = bilateralClient.updateAsk(
        updateAsk = updateAsk,
        signer = signer,
    ).also { updateResponse ->
        val askId = updateAsk.ask.mapToId()
        assertEquals(
            expected = askId,
            actual = updateResponse.askId,
            message = "The correct ask id was not returned in the update ask response",
        )
        assertEquals(
            expected = bilateralClient.assertAskExists(askId),
            actual = updateResponse.updatedAskOrder,
            message = "The ask order stored in the contract does not match the value returned in the update ask response",
        )
    }

    fun cancelAsk(
        askId: String,
        signer: Signer = asker,
    ): CancelAskResponse = bilateralClient.cancelAsk(
        askId = askId,
        signer = signer,
    ).also { cancelResponse ->
        assertEquals(
            expected = askId,
            actual = cancelResponse.askId,
            message = "The correct ask id was not returned in the cancel ask response",
        )
        bilateralClient.assertAskIsDeleted(askId)
        createdAsks.remove(askId)
    }

    fun createBid(
        createBid: CreateBid,
        signer: Signer = bidder,
    ): CreateBidResponse = bilateralClient.createBid(
        createBid = createBid,
        signer = signer,
    ).also { createResponse ->
        val bidId = createBid.bid.mapToId()
        assertEquals(
            expected = bidId,
            actual = createResponse.bidId,
            message = "The correct bid id was not returned in the create bid response",
        )
        assertEquals(
            expected = bilateralClient.assertBidExists(bidId),
            actual = createResponse.bidOrder,
            message = "The bid order stored in the contract does not match the value returned in the bid response",
        )
        createdBids += CreatedBid(bidId, signer).let { bidId to it }
    }

    fun updateBid(
        updateBid: UpdateBid,
        signer: Signer = bidder,
    ): UpdateBidResponse = bilateralClient.updateBid(
        updateBid = updateBid,
        signer = signer,
    ).also { updateResponse ->
        val bidId = updateBid.bid.mapToId()
        assertEquals(
            expected = bidId,
            actual = updateResponse.bidId,
            message = "The correct bid id was not returned in the update bid response",
        )
        assertEquals(
            expected = bilateralClient.assertBidExists(updateBid.bid.mapToId()),
            actual = updateResponse.updatedBidOrder,
            message = "The bid order stored in the contract does not match the value returned in the update bid response",
        )
    }

    fun cancelBid(
        bidId: String,
        signer: Signer = bidder,
    ): CancelBidResponse = bilateralClient.cancelBid(
        bidId = bidId,
        signer = signer,
    ).also { cancelResponse ->
        assertEquals(
            expected = bidId,
            actual = cancelResponse.bidId,
            message = "The correct bid id was not returned in the cancel bid response",
        )
        bilateralClient.assertBidIsDeleted(bidId)
        createdBids.remove(bidId)
    }

    fun executeMatch(
        executeMatch: ExecuteMatch,
        signer: Signer = admin,
    ): ExecuteMatchResponse = bilateralClient.executeMatch(
        executeMatch = executeMatch,
        signer = signer,
    ).also { executeResponse ->
        assertEquals(
            expected = executeMatch.askId,
            actual = executeResponse.askId,
            message = "The correct ask id was not returned in the execute match response",
        )
        assertEquals(
            expected = executeMatch.bidId,
            actual = executeResponse.bidId,
            message = "The correct bid id was not returned in the execute match response",
        )
        if (executeResponse.askDeleted) {
            bilateralClient.assertAskIsDeleted(executeMatch.askId)
            logger.info("[TESTCLEANUP]: Removing storage record for deleted ask [${executeResponse.askId}]")
            createdAsks.remove(executeResponse.askId)
        } else {
            bilateralClient.assertAskExists(executeMatch.askId)
        }
        if (executeResponse.bidDeleted) {
            bilateralClient.assertBidIsDeleted(executeMatch.bidId)
            logger.info("[TESTCLEANUP]: Removing storage record for deleted bid [${executeResponse.bidId}]")
            createdBids.remove(executeResponse.bidId)
        } else {
            bilateralClient.assertBidExists(executeMatch.bidId)
        }
    }

    private fun entryCountDisplay(map: Map<*, *>): String = "${map.size} ${if (map.size == 1) "entry" else "entries"}"
}

private data class CreatedAsk(
    val askId: String,
    val signer: Signer,
)

private data class CreatedBid(
    val bidId: String,
    val signer: Signer,
)
