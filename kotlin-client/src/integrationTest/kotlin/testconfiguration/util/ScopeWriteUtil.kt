package testconfiguration.util

import cosmos.tx.v1beta1.ServiceOuterClass.BroadcastMode
import io.provenance.client.grpc.BaseReqSigner
import io.provenance.client.grpc.PbClient
import io.provenance.client.grpc.Signer
import io.provenance.client.protobuf.extensions.toAny
import io.provenance.client.protobuf.extensions.toTxBody
import io.provenance.metadata.v1.MsgWriteScopeRequest
import io.provenance.metadata.v1.Party
import io.provenance.metadata.v1.PartyType
import io.provenance.scope.util.MetadataAddress
import io.provenance.scope.util.toByteString
import io.provenance.spec.AssetSpecifications
import io.provenance.spec.HELOCSpecification
import mu.KotlinLogging
import testconfiguration.accounts.BilateralAccounts
import testconfiguration.extensions.checkIsSuccess
import java.util.UUID

object ScopeWriteUtil {
    private val logger = KotlinLogging.logger {}

    fun writeInitialScopeSpecs(pbClient: PbClient) {
        val writeSpecMsgs = AssetSpecifications.flatMap { it.specificationMsgs(BilateralAccounts.fundingAccount.address()) }
        logger.info("Writing initial scope specifications, exposing all of AssetSpecifications as viable targets")
        pbClient.estimateAndBroadcastTx(
            txBody = writeSpecMsgs.map { it.toAny() }.toTxBody(),
            signers = BilateralAccounts.fundingAccount.let(::BaseReqSigner).let(::listOf),
            mode = BroadcastMode.BROADCAST_MODE_BLOCK,
            gasAdjustment = 1.3,
        ).checkIsSuccess().also {
            logger.info("Successfully wrote initial scope specs")
        }
    }

    fun writeMockScope(
        pbClient: PbClient,
        signer: Signer,
        scopeUuid: UUID = UUID.randomUUID(),
        specUuid: UUID = HELOCSpecification.scopeSpecConfig.id,
        ownerAddress: String = signer.address(),
        valueOwnerAddress: String = signer.address(),
    ): ScopeCreationDetail {
        val writeScopeMsg = MsgWriteScopeRequest.newBuilder().also { req ->
            req.scopeUuid = scopeUuid.toString()
            req.specUuid = specUuid.toString()
            req.addSigners(signer.address())
            req.scopeBuilder.scopeId = MetadataAddress.forScope(scopeUuid).bytes.toByteString()
            req.scopeBuilder.specificationId = MetadataAddress.forScopeSpecification(specUuid).bytes.toByteString()
            req.scopeBuilder.valueOwnerAddress = valueOwnerAddress
            req.scopeBuilder.addOwners(
                Party.newBuilder().also { party ->
                    party.address = ownerAddress
                    party.role = PartyType.PARTY_TYPE_OWNER
                }
            )
            req.scopeBuilder.addAllDataAccess(listOf(signer.address(), ownerAddress, valueOwnerAddress).distinct())
        }.build().toAny()
        logger.info("Creating scope [$scopeUuid] with owner [$ownerAddress] and value owner [$valueOwnerAddress], signed by [${signer.address()}]")
        pbClient.estimateAndBroadcastTx(
            txBody = writeScopeMsg.toTxBody(),
            signers = BaseReqSigner(signer).let(::listOf),
            mode = BroadcastMode.BROADCAST_MODE_BLOCK,
        ).checkIsSuccess().also { logger.info("Successfully created scope [$scopeUuid]") }
        return ScopeCreationDetail(scopeUuid = scopeUuid, scopeSpecUuid = specUuid)
    }
}

data class ScopeCreationDetail(
    val scopeUuid: UUID,
    val scopeSpecUuid: UUID,
)
