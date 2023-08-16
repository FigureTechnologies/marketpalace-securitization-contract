package testconfiguration.functions

import cosmos.base.v1beta1.CoinOuterClass.Coin
import cosmos.tx.v1beta1.ServiceOuterClass.BroadcastMode
import io.provenance.client.grpc.BaseReqSigner
import io.provenance.client.grpc.PbClient
import io.provenance.client.grpc.Signer
import io.provenance.client.protobuf.extensions.toAny
import io.provenance.client.protobuf.extensions.toTxBody
import io.provenance.marker.v1.Access
import io.provenance.marker.v1.AccessGrant
import io.provenance.marker.v1.MarkerStatus
import io.provenance.marker.v1.MarkerType
import io.provenance.marker.v1.MsgActivateRequest
import io.provenance.marker.v1.MsgAddAccessRequest
import io.provenance.marker.v1.MsgAddMarkerRequest
import io.provenance.marker.v1.MsgWithdrawRequest
import io.provenance.name.v1.MsgBindNameRequest
import io.provenance.name.v1.NameRecord
import mu.KotlinLogging
import testconfiguration.accounts.BilateralAccounts
import testconfiguration.extensions.checkIsSuccess
import testconfiguration.extensions.getBalance
import kotlin.test.assertEquals

private val logger = KotlinLogging.logger("ProvenanceFunctions")

fun newCoin(amount: Long, denom: String): Coin = Coin.newBuilder().setAmount(amount.toString()).setDenom(denom).build()

fun newCoins(amount: Long, denom: String): List<Coin> = listOf(newCoin(amount, denom))

fun createMarker(
    pbClient: PbClient,
    ownerAccount: Signer,
    denomName: String,
    supply: Long,
    fixed: Boolean = true,
    allowGovControl: Boolean = true,
    // Mimics the grants given in asset manager
    permissions: List<Access> = listOf(
        Access.ACCESS_ADMIN,
        Access.ACCESS_DEPOSIT,
        Access.ACCESS_WITHDRAW,
        Access.ACCESS_BURN,
        Access.ACCESS_MINT,
        Access.ACCESS_DELETE,
    )
) {
    val addReq = MsgAddMarkerRequest.newBuilder().also { req ->
        req.amount = newCoin(supply, denomName)
        req.fromAddress = ownerAccount.address()
        req.markerType = MarkerType.MARKER_TYPE_COIN
        req.status = MarkerStatus.MARKER_STATUS_FINALIZED
        req.supplyFixed = fixed
        req.allowGovernanceControl = allowGovControl
        req.addAccessList(
            AccessGrant.newBuilder().also { grant ->
                grant.address = ownerAccount.address()
                grant.addAllPermissions(permissions)
            }
        )
    }.build()
    val activateReq = MsgActivateRequest.newBuilder().also { req ->
        req.administrator = ownerAccount.address()
        req.denom = denomName
    }.build()
    logger.info("Creating marker [$denomName] with admin address [${ownerAccount.address()}]")
    pbClient.estimateAndBroadcastTx(
        txBody = listOf(addReq, activateReq).map { it.toAny() }.toTxBody(),
        signers = listOf(BaseReqSigner(ownerAccount)),
        mode = BroadcastMode.BROADCAST_MODE_BLOCK,
        gasAdjustment = 1.3,
    ).checkIsSuccess().also { logger.info("Successfully created marker [$denomName] for owner [${ownerAccount.address()}]") }
}

fun giveTestDenom(
    pbClient: PbClient,
    initialHoldings: Coin,
    receiverAddress: String,
    sendAmount: Long = initialHoldings.amount.toLong(),
) {
    createMarker(pbClient = pbClient, ownerAccount = BilateralAccounts.fundingAccount, denomName = initialHoldings.denom, supply = initialHoldings.amount.toLong())
    val msgWithdraw = MsgWithdrawRequest.newBuilder().also { withdraw ->
        withdraw.administrator = BilateralAccounts.fundingAccount.address()
        withdraw.addAmount(initialHoldings.toBuilder().setAmount(sendAmount.toString()).build())
        withdraw.denom = initialHoldings.denom
        withdraw.toAddress = receiverAddress
    }.build().toAny()
    logger.info("Sending [$sendAmount${initialHoldings.denom}] to [$receiverAddress] from marker admin [${BilateralAccounts.fundingAccount.address()}]")
    pbClient.estimateAndBroadcastTx(
        txBody = msgWithdraw.toTxBody(),
        signers = listOf(BaseReqSigner(BilateralAccounts.fundingAccount)),
        mode = BroadcastMode.BROADCAST_MODE_BLOCK,
        gasAdjustment = 1.3,
    ).checkIsSuccess().also {
        logger.info("Successfully sent [$sendAmount${initialHoldings.denom}] to [$receiverAddress]")
    }
    assertEquals(
        expected = sendAmount,
        actual = pbClient.getBalance(receiverAddress, initialHoldings.denom),
        message = "The receiver account [$receiverAddress] should have [$sendAmount${initialHoldings.denom}] after the withdraw",
    )
}

fun grantMarkerAccess(
    pbClient: PbClient,
    markerAdminAccount: Signer,
    markerDenom: String,
    grantAddress: String,
    permissions: List<Access> = listOf(Access.ACCESS_ADMIN),
) {
    val accessReq = MsgAddAccessRequest.newBuilder().also { req ->
        req.denom = markerDenom
        req.administrator = markerAdminAccount.address()
        req.addAccess(
            AccessGrant.newBuilder().also { grant ->
                grant.address = grantAddress
                grant.addAllPermissions(permissions)
            }
        )
    }.build()
    logger.info("Granting access $permissions to account [$grantAddress] using admin address [${markerAdminAccount.address()}] for marker [$markerDenom]")
    pbClient.estimateAndBroadcastTx(
        txBody = accessReq.toAny().toTxBody(),
        signers = listOf(BaseReqSigner(markerAdminAccount)),
        mode = BroadcastMode.BROADCAST_MODE_BLOCK,
        gasAdjustment = 1.3,
    ).checkIsSuccess().also { logger.info("Granted access $permissions to account [$grantAddress] on marker [$markerDenom]") }
}

fun bindNamesToSigner(
    pbClient: PbClient,
    names: List<String>,
    signer: Signer,
    restricted: Boolean,
) {
    names.map { name ->
        MsgBindNameRequest.newBuilder().also { bindName ->
            val nameParts = name.split(".")
            val rootName = nameParts.first()
            val parentName = nameParts.drop(1).joinToString("")
            bindName.parent = NameRecord.newBuilder().also { record ->
                record.name = parentName
                record.address = signer.address()
            }.build()
            bindName.record = NameRecord.newBuilder().also { record ->
                record.name = rootName
                record.address = signer.address()
                record.restricted = restricted
            }.build()
        }.build().toAny()
    }.also { nameMsgs ->
        logger.info("Binding names $names to account [${signer.address()}] with restricted=$restricted")
        pbClient.estimateAndBroadcastTx(
            txBody = nameMsgs.toTxBody(),
            signers = listOf(BaseReqSigner(signer)),
            mode = BroadcastMode.BROADCAST_MODE_BLOCK,
            gasAdjustment = 1.3,
        ).checkIsSuccess().also { logger.info("Successfully bound names $names to account [${signer.address()}]") }
    }
}
