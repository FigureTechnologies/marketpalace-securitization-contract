package testconfiguration.testcontainers

import io.provenance.client.grpc.GasEstimationMethod
import io.provenance.client.grpc.PbClient
import io.provenance.client.protobuf.extensions.getBaseAccount
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import mu.KLogging
import org.testcontainers.containers.ContainerLaunchException
import org.testcontainers.containers.GenericContainer
import org.testcontainers.containers.wait.strategy.AbstractWaitStrategy
import testconfiguration.util.CoroutineUtil
import java.net.URI

class ProvenanceTestContainer : GenericContainer<ProvenanceTestContainer>("provenanceio/provenance:v1.11.1") {
    private companion object : KLogging()

    init {
        logger.info("Starting Provenance Blockchain container version v1.11.1")
    }
}

class ProvenanceWaitStrategy(private val expectedGenesisAccountBech32: String) : AbstractWaitStrategy() {
    private companion object : KLogging()

    override fun waitUntilReady() {
        try {
            val host = waitStrategyTarget.host
            val port = waitStrategyTarget.getMappedPort(9090)
            logger.info("Starting PbClient at $host:$port[mapped from port 9090]")
            val pbClient = getPbClient(host = host, mappedPort = port)
            logger.info("Checking for genesis account [$expectedGenesisAccountBech32] existence...")
            runBlocking {
                launch {
                    val account = CoroutineUtil.withRetryBackoff(
                        errorPrefix = "Waiting for genesis account [$expectedGenesisAccountBech32] to be created",
                        initialDelay = 1000L,
                        maxDelay = 20000L,
                        showStackTraceInFailures = false,
                        block = { pbClient.authClient.getBaseAccount(expectedGenesisAccountBech32) },
                    )
                    logger.info("Successfully fetched genesis account [${account.address}] with account number [${account.accountNumber}]")
                }.join()
            }
            logger.info("Closing the PbClient instance...")
            pbClient.close()
        } catch (e: Exception) {
            throw ContainerLaunchException("Provenance was not in a healthy state", e)
        }
    }
}

private fun getPbClient(host: String, mappedPort: Int): PbClient = PbClient(
    chainId = "chain-local",
    channelUri = URI.create("http://$host:$mappedPort"),
    gasEstimationMethod = GasEstimationMethod.MSG_FEE_CALCULATION,
)
