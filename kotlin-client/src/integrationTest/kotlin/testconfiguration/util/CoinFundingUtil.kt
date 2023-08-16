package testconfiguration.util

import cosmos.bank.v1beta1.Tx.MsgSend
import cosmos.base.v1beta1.CoinOuterClass.Coin
import cosmos.tx.v1beta1.ServiceOuterClass.BroadcastMode
import io.provenance.client.grpc.BaseReqSigner
import io.provenance.client.grpc.PbClient
import io.provenance.client.grpc.Signer
import io.provenance.client.protobuf.extensions.toAny
import io.provenance.client.protobuf.extensions.toTxBody
import mu.KotlinLogging
import testconfiguration.extensions.checkIsSuccess

object CoinFundingUtil {
    private val logger = KotlinLogging.logger {}

    fun fundAccounts(
        pbClient: PbClient,
        senderAccount: Signer,
        receiverAccounts: List<Signer>,
        fundingAmount: Long = 20_000_000_000_000,
        fundingDenom: String = "nhash",
    ) {
        val sendMsgs = receiverAccounts.map { receiver ->
            MsgSend.newBuilder().also { send ->
                send.fromAddress = senderAccount.address()
                send.toAddress = receiver.address()
                send.addAmount(Coin.newBuilder().setAmount(fundingAmount.toString()).setDenom(fundingDenom).build())
            }.build().toAny()
        }
        logger.info("Sending funding messages to add [${fundingAmount}$fundingDenom] to accounts ${receiverAccounts.map { it.address() }} from [${senderAccount.address()}]")
        pbClient.estimateAndBroadcastTx(
            txBody = sendMsgs.toTxBody(),
            signers = listOf(BaseReqSigner(senderAccount)),
            mode = BroadcastMode.BROADCAST_MODE_BLOCK,
            gasAdjustment = 1.3,
        ).checkIsSuccess()
        logger.info("Successfully funded accounts ${receiverAccounts.map { it.address() }} from [${senderAccount.address()}]")
    }
}
