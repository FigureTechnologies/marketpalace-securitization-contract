package testconfiguration.util

import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonNaming
import com.fasterxml.jackson.databind.annotation.JsonSerialize
import cosmos.tx.v1beta1.ServiceOuterClass.BroadcastMode
import cosmwasm.wasm.v1.Tx.MsgInstantiateContract
import cosmwasm.wasm.v1.Tx.MsgStoreCode
import cosmwasm.wasm.v1.Types.AccessConfig
import cosmwasm.wasm.v1.Types.AccessType
import io.provenance.bilateral.interfaces.BilateralContractMsg
import io.provenance.bilateral.serialization.CosmWasmBigIntegerToUintSerializer
import io.provenance.client.grpc.BaseReqSigner
import io.provenance.client.grpc.PbClient
import io.provenance.client.grpc.Signer
import io.provenance.client.protobuf.extensions.toAny
import io.provenance.client.protobuf.extensions.toTxBody
import io.provenance.scope.util.toByteString
import mu.KotlinLogging
import testconfiguration.extensions.getCodeId
import testconfiguration.extensions.getContractAddress
import java.io.ByteArrayOutputStream
import java.math.BigInteger
import java.util.zip.GZIPOutputStream

object BilateralSmartContractUtil {
    private val logger = KotlinLogging.logger {}
    private const val CONTRACT_NAME: String = "metadatabilateral.pb"

    fun instantiateSmartContract(pbClient: PbClient, contractAdmin: Signer): ContractInstantiationResult {
        logger.info("Fetching and gzipping wasm bytes from resources")
        val wasmBytes = getGzippedWasmBytes()
        return storeAndInstantiate(pbClient, contractAdmin, wasmBytes)
    }

    private fun getGzippedWasmBytes(): ByteArray = ClassLoader
        .getSystemResource("artifacts/metadata_bilateral_exchange.wasm")
        .readBytes()
        .let { wasmBytes ->
            ByteArrayOutputStream().use { byteStream ->
                GZIPOutputStream(byteStream).use { it.write(wasmBytes, 0, wasmBytes.size) }
                byteStream.toByteArray()
            }
        }

    private fun storeAndInstantiate(
        pbClient: PbClient,
        contractAdmin: Signer,
        wasmBytes: ByteArray
    ): ContractInstantiationResult {
        val storeCodeMsg = MsgStoreCode.newBuilder().also { storeCode ->
            storeCode.instantiatePermission = AccessConfig.newBuilder().also { accessConfig ->
                accessConfig.address = contractAdmin.address()
                accessConfig.permission = AccessType.ACCESS_TYPE_ONLY_ADDRESS
            }.build()
            storeCode.sender = contractAdmin.address()
            storeCode.wasmByteCode = wasmBytes.toByteString()
        }.build().toAny()
        logger.info("Storing code on Provenance Blockchain using address [${contractAdmin.address()}] as the contract admin")
        val codeId = pbClient.estimateAndBroadcastTx(
            txBody = storeCodeMsg.toTxBody(),
            signers = listOf(BaseReqSigner(signer = contractAdmin)),
            gasAdjustment = 1.1,
            mode = BroadcastMode.BROADCAST_MODE_BLOCK,
        ).getCodeId()
        logger.info("Successfully stored contract and got code id [$codeId]")
        val instantiateMsg = MsgInstantiateContract.newBuilder().also { instantiate ->
            instantiate.admin = contractAdmin.address()
            instantiate.codeId = codeId
            instantiate.label = "metadata-bilateral-exchange"
            instantiate.sender = contractAdmin.address()
            instantiate.msg = MetadataBilateralExchangeInstantiateMsg(
                bindName = CONTRACT_NAME,
                contractName = "Metadata Bilateral Exchange",
            ).toJsonByteString(ObjectMapperProvider.OBJECT_MAPPER)
        }.build().toAny()
        logger.info("Instantiating contract on Provenance Blockchain with code id [$codeId] and contract name [$CONTRACT_NAME]")
        val contractAddress = pbClient.estimateAndBroadcastTx(
            txBody = instantiateMsg.toTxBody(),
            signers = listOf(BaseReqSigner(signer = contractAdmin)),
            gasAdjustment = 1.1,
            mode = BroadcastMode.BROADCAST_MODE_BLOCK,
        ).getContractAddress()
        logger.info("Successfully stored metadata bilateral exchange contract using code id [$codeId], name [$CONTRACT_NAME] and got address [$contractAddress]")
        return ContractInstantiationResult(
            codeId = codeId,
            contractBindingName = CONTRACT_NAME,
            contractAddress = contractAddress,
        )
    }
}

@JsonNaming(SnakeCaseStrategy::class)
data class MetadataBilateralExchangeInstantiateMsg(
    val bindName: String,
    val contractName: String,
    @JsonSerialize(using = CosmWasmBigIntegerToUintSerializer::class)
    val createAskNhashFee: BigInteger? = null,
    @JsonSerialize(using = CosmWasmBigIntegerToUintSerializer::class)
    val createBidNhashFee: BigInteger? = null,
) : BilateralContractMsg {
    override fun toLoggingString(): String = "metadataBilateralExchangeInstantiateMsg, bindName = [$bindName], contractName = [$contractName], creatAskNhashFee = [$createAskNhashFee], createBidNhashFee = [$createBidNhashFee]"
}

data class ContractInstantiationResult(
    val codeId: Long,
    val contractBindingName: String,
    val contractAddress: String,
)
