package testconfiguration.extensions

import com.fasterxml.jackson.core.type.TypeReference
import cosmos.bank.v1beta1.QueryOuterClass.QueryAllBalancesRequest
import cosmos.bank.v1beta1.QueryOuterClass.QueryBalanceRequest
import cosmos.bank.v1beta1.Tx.MsgSend
import cosmos.base.abci.v1beta1.Abci.TxResponse
import cosmos.base.v1beta1.CoinOuterClass.Coin
import cosmos.tx.v1beta1.ServiceOuterClass.BroadcastMode
import cosmos.tx.v1beta1.ServiceOuterClass.BroadcastTxResponse
import io.provenance.client.grpc.BaseReqSigner
import io.provenance.client.grpc.PbClient
import io.provenance.client.grpc.Signer
import io.provenance.client.protobuf.extensions.toAny
import io.provenance.client.protobuf.extensions.toTxBody
import io.provenance.marker.v1.MarkerAccount
import io.provenance.marker.v1.QueryMarkerRequest
import mu.KotlinLogging
import testconfiguration.models.ProvenanceTxEvents
import testconfiguration.util.ObjectMapperProvider.OBJECT_MAPPER

private val logger = KotlinLogging.logger("ProvenanceExtensions")

fun TxResponse.isError(): Boolean = this.code != 0
fun TxResponse.isSuccess(): Boolean = !this.isError()
fun TxResponse.checkIsSuccess(): TxResponse = this.also {
    if (this.isError()) {
        throw IllegalStateException("Broadcast response contained an error! Error log: ${this.rawLog}")
    }
}

/**
 * Expands the rawLow property of the TxResponse into a navigable data structure for logical parsing.
 * When the log corresponds to an error, an exception will be thrown.
 */
fun TxResponse.toProvenanceTxEvents(): List<ProvenanceTxEvents> = OBJECT_MAPPER
    .readValue(this.checkIsSuccess().rawLog, object : TypeReference<List<ProvenanceTxEvents>>() {})

fun BroadcastTxResponse.toProvenanceTxEvents(): List<ProvenanceTxEvents> = txResponse.toProvenanceTxEvents()

fun BroadcastTxResponse.isError(): Boolean = this.txResponse.isError()
fun BroadcastTxResponse.isSuccess(): Boolean = !this.isError()
fun BroadcastTxResponse.checkIsSuccess(): BroadcastTxResponse = this.also { this.txResponse.checkIsSuccess() }

fun BroadcastTxResponse.getCodeIdOrNull(): Long? = toProvenanceTxEvents()
    .flatMap { it.events }
    .singleOrNull { it.type == "store_code" }
    ?.attributes
    ?.singleOrNull { it.key == "code_id" }
    ?.value
    ?.toLongOrNull()

fun BroadcastTxResponse.getCodeId(): Long = getCodeIdOrNull()
    ?: throw IllegalStateException("Unable to retrieve code id from response. Received response log: ${this.txResponse.rawLog}")

fun BroadcastTxResponse.getContractAddressOrNull(): String? = toProvenanceTxEvents()
    .flatMap { it.events }
    .singleOrNull { it.type == "instantiate" }
    ?.attributes
    ?.singleOrNull { it.key == "_contract_address" }
    ?.value

fun BroadcastTxResponse.getContractAddress(): String = getContractAddressOrNull()
    ?: throw IllegalStateException("Unable to retrieve contract address from response.  Received response log: ${this.txResponse.rawLog}")

fun PbClient.sendCoin(coin: Coin, fromSigner: Signer, toAddress: String) {
    val send = MsgSend.newBuilder().also { send ->
        send.fromAddress = fromSigner.address()
        send.toAddress = toAddress
        send.addAmount(coin)
    }.build().toAny()
    logger.info("Sending [${coin.amount}${coin.denom}] from [${fromSigner.address()}] to [$toAddress]")
    this.estimateAndBroadcastTx(
        txBody = send.toTxBody(),
        signers = listOf(BaseReqSigner(fromSigner)),
        mode = BroadcastMode.BROADCAST_MODE_BLOCK,
        gasAdjustment = 1.1,
    ).checkIsSuccess().also {
        logger.info("[$toAddress] successfully received [${coin.amount}${coin.denom}]")
    }
}

fun PbClient.getBalanceMap(accountAddress: String): Map<String, Long> = bankClient
    .allBalances(QueryAllBalancesRequest.newBuilder().setAddress(accountAddress).build())
    .balancesList
    .associate { coin -> coin.denom to coin.amount.toLong() }

fun PbClient.getBalance(accountAddress: String, denom: String): Long = bankClient
    .balance(QueryBalanceRequest.newBuilder().setDenom(denom).setAddress(accountAddress).build())
    .balance
    .amount
    // A missing balance will show up as zero so this is safe
    .toLong()

fun PbClient.getMarkerAccount(markerDenom: String): MarkerAccount = markerClient
    .marker(QueryMarkerRequest.newBuilder().setId(markerDenom).build())
    .marker
    .unpack(MarkerAccount::class.java)
