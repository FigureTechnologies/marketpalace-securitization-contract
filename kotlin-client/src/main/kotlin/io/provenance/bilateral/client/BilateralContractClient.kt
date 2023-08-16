package io.provenance.bilateral.client

import com.fasterxml.jackson.core.type.TypeReference
import com.fasterxml.jackson.databind.ObjectMapper
import cosmos.base.abci.v1beta1.Abci.TxResponse
import cosmos.base.v1beta1.CoinOuterClass.Coin
import cosmwasm.wasm.v1.QueryOuterClass
import cosmwasm.wasm.v1.Tx.MsgExecuteContract
import io.provenance.bilateral.exceptions.NullContractResultException
import io.provenance.bilateral.exceptions.ProvenanceEventParsingException
import io.provenance.bilateral.execute.CancelAsk
import io.provenance.bilateral.execute.CancelBid
import io.provenance.bilateral.execute.CreateAsk
import io.provenance.bilateral.execute.CreateBid
import io.provenance.bilateral.execute.ExecuteMatch
import io.provenance.bilateral.execute.UpdateAsk
import io.provenance.bilateral.execute.UpdateBid
import io.provenance.bilateral.execute.UpdateSettings
import io.provenance.bilateral.extensions.booleanAttribute
import io.provenance.bilateral.extensions.executeContractDataToJsonBytes
import io.provenance.bilateral.extensions.singleWasmEvent
import io.provenance.bilateral.extensions.stringAttribute
import io.provenance.bilateral.extensions.stringAttributeOrNull
import io.provenance.bilateral.interfaces.BilateralContractExecuteMsg
import io.provenance.bilateral.interfaces.BilateralContractQueryMsg
import io.provenance.bilateral.models.AdminMatchOptions
import io.provenance.bilateral.models.AskOrder
import io.provenance.bilateral.models.BidOrder
import io.provenance.bilateral.models.ContractInfo
import io.provenance.bilateral.models.ContractSearchResult
import io.provenance.bilateral.models.MatchReport
import io.provenance.bilateral.models.executeresponse.CancelAskResponse
import io.provenance.bilateral.models.executeresponse.CancelBidResponse
import io.provenance.bilateral.models.executeresponse.CreateAskResponse
import io.provenance.bilateral.models.executeresponse.CreateBidResponse
import io.provenance.bilateral.models.executeresponse.ExecuteMatchResponse
import io.provenance.bilateral.models.executeresponse.UpdateAskResponse
import io.provenance.bilateral.models.executeresponse.UpdateBidResponse
import io.provenance.bilateral.models.executeresponse.UpdateSettingsResponse
import io.provenance.bilateral.query.ContractSearchRequest
import io.provenance.bilateral.query.GetAsk
import io.provenance.bilateral.query.GetAsksByCollateralId
import io.provenance.bilateral.query.GetBid
import io.provenance.bilateral.query.GetContractInfo
import io.provenance.bilateral.query.GetMatchReport
import io.provenance.bilateral.util.ContractAddressResolver
import io.provenance.bilateral.util.ObjectMapperProvider
import io.provenance.client.grpc.BaseReqSigner
import io.provenance.client.grpc.PbClient
import io.provenance.client.grpc.Signer
import io.provenance.client.protobuf.extensions.queryWasm
import io.provenance.client.protobuf.extensions.toAny
import io.provenance.client.protobuf.extensions.toTxBody
import tendermint.abci.Types.Event

/**
 * A client object that makes GRPC requests to the Provenance Blockchain by leveraging a provided PbClient in order to
 * properly interact with the various json structures required by the Metadata Bilateral Exchange smart contract.
 *
 * @param pbClient The core communication functionality of this client.  This is used to execute and query the smart
 * contract.
 * @param addressResolver Provides a way of locating an instance of the Metadata Bilateral Exchange smart contract on the
 * target Provenance Blockchain instance.
 * @param objectMapper Provides the functionality needed to convert request data classes to json for interaction with
 * the smart contract, and to convert response data from the smart contract to response data classes.  If not provided
 * in the builder, a default value that can properly perform these functions is provided.
 * @param logger Provides a method for the client to log information for the consumer.  If not provided, the default
 * behavior is to ignore log input and do nothing. Ex: [BilateralContractClientLogger.Off].
 */
class BilateralContractClient private constructor(
    private val pbClient: PbClient,
    private val addressResolver: ContractAddressResolver,
    private val objectMapper: ObjectMapper,
    private val logger: BilateralContractClientLogger,
) {
    companion object {
        /**
         * Constructs a builder class for the []BilateralContractClient].
         *
         * @param pbClient The core communication functionality of this client.  This is used to execute and query the smart
         * contract.  This value is required because it is impossible to target a Provenance Blockchain instance without
         * a PbClient instance that targets one.
         * @param addressResolver Provides a way of locating an instance of the Metadata Bilateral Exchange smart contract on the
         * target Provenance Blockchain instance. This value is required because it is impossible to target an instance
         * of the Metadata Bilateral Exchange smart contract without a bech32 address reference.
         */
        fun builder(pbClient: PbClient, addressResolver: ContractAddressResolver): BilateralContractClientBuilder =
            BilateralContractClientBuilder(pbClient = pbClient, addressResolver = addressResolver)
    }

    /**
     * A builder class that constructs an instance of the [BilateralContractClient] when its [BilateralContractClientBuilder.build]
     * function is invoked.
     *
     * @param pbClient The core communication functionality of this client.  This is used to execute and query the smart
     * contract.
     * @param addressResolver Provides a way of locating an instance of the Metadata Bilateral Exchange smart contract on the
     * target Provenance Blockchain instance.
     */
    class BilateralContractClientBuilder(
        private val pbClient: PbClient,
        private val addressResolver: ContractAddressResolver
    ) {
        private var objectMapper: ObjectMapper? = null
        private var logger: BilateralContractClientLogger? = null

        /**
         * Provides a custom [com.fasterxml.jackson.databind.ObjectMapper] instance to the [BilateralContractClient].
         */
        fun setObjectMapper(objectMapper: ObjectMapper) = apply { this.objectMapper = objectMapper }

        /**
         * Provides a [BilateralContractClientLogger] instance.  This essentially enables logging, because the default
         * behavior is to ignore all logging.
         */
        fun setLogger(logger: BilateralContractClientLogger) = apply { this.logger = logger }

        /**
         * Builds an instance of the [BilateralContractClient], providing default values if none were provided during
         * the build pattern.
         */
        fun build(): BilateralContractClient = BilateralContractClient(
            pbClient = pbClient,
            addressResolver = addressResolver,
            objectMapper = objectMapper ?: ObjectMapperProvider.OBJECT_MAPPER,
            logger = logger ?: BilateralContractClientLogger.Off,
        )
    }

    /**
     * The exposed bech32 address of the targeted Metadata Bilateral Exchange smart contract, after a successful
     * resolution.  If the contract cannot be located by address, an exception will be thrown upon the first communication
     * with the contract via the client.
     */
    val contractAddress by lazy { addressResolver.getAddress(pbClient) }

    /**
     * Fetches an AskOrder from the contract's storage by id.  Throws an exception if the ask cannot be found or if
     * Provenance Blockchain communications fail.
     *
     * @param id The unique id of the AskOrder to locate.
     */
    fun getAsk(id: String): AskOrder = queryContract(
        query = GetAsk(id = id),
    )

    /**
     * Fetches an AskOrder from the contract's storage by id.  Returns null if the ask cannot be found or if
     * Provenance Blockchain communications fail.
     *
     * @param id The unique id of the AskOrder to locate.
     */
    fun getAskOrNull(id: String): AskOrder? = queryContractOrNull(
        query = GetAsk(id = id),
    )

    /**
     * Fetches AskOrders from the contract's storage by their collateral id.  See [io.provenance.bilateral.query.GetAsksByCollateralId]
     * for each different collateral id type.  Throws an exception if Provenance Blockchain communications fail.
     *
     * @param collateralId The collateral id of the AskOrders to locate.
     */
    fun getAsksByCollateralId(collateralId: String): List<AskOrder> = queryContract(
        query = GetAsksByCollateralId(collateralId = collateralId),
        typeReference = object : TypeReference<List<AskOrder>>() {},
    )

    /**
     * Fetches AskOrders from the contract's storage by their collateral id.  See [io.provenance.bilateral.query.GetAsksByCollateralId]
     * for each different collateral id type.  Returns null if Provenance Blockchain communications fail.
     *
     * @param collateralId The collateral id of the AskOrders to locate.
     */
    fun getAsksByCollateralIdOrNull(collateralId: String): List<AskOrder>? = queryContractOrNull(
        query = GetAsksByCollateralId(collateralId = collateralId),
        typeReference = object : TypeReference<List<AskOrder>>() {},
    )

    /**
     * Fetches a BidOrder from the contract's storage by id.  Throws an exception if the bid cannot be found or if
     * Provenance Blockchain communications fail.
     *
     * @param id The unique id of the BidOrder to locate.
     */
    fun getBid(id: String): BidOrder = queryContract(
        query = GetBid(id = id),
    )

    /**
     * Fetches a BidOrder from the contract's storage by id.  Returns null if the bid cannot be found or if
     * Provenance Blockchain communications fail.
     *
     * @param id The unique id of the BidOrder to locate.
     */
    fun getBidOrNull(id: String): BidOrder? = queryContractOrNull(
        query = GetBid(id = id),
    )

    /**
     * Simulates a match between an ask and a bid, determining if the match will be accepted or rejected.  If the ask
     * or bid is missing, that will also be indicated in the report and not cause an exception.  Throws an exception if
     * Provenance Blockchain communications fail.
     *
     * @param askId The unique id of the ask to execute in the simulated match.
     * @param bidId The unique id of the bid to execute in the simulated match.
     * @param adminMatchOptions Various options that alter the behavior of the matching mechanism.
     */
    fun getMatchReport(
        askId: String,
        bidId: String,
        adminMatchOptions: AdminMatchOptions? = null,
    ): MatchReport = queryContract(
        query = GetMatchReport(askId = askId, bidId = bidId, adminMatchOptions = adminMatchOptions),
    )

    /**
     * Simulates a match between an ask and a bid, determining if the match will be accepted or rejected.  If the ask
     * or bid is missing, that will also be indicated in the report and not cause a null result.  Returns null if
     * Provenance Blockchain communications fail.
     *
     * @param askId The unique id of the ask to execute in the simulated match.
     * @param bidId The unique id of the bid to execute in the simulated match.
     * @param adminMatchOptions Various options that alter the behavior of the matching mechanism.
     */
    fun getMatchReportOrNull(
        askId: String,
        bidId: String,
        adminMatchOptions: AdminMatchOptions? = null,
    ): MatchReport? = queryContractOrNull(
        query = GetMatchReport(askId = askId, bidId = bidId, adminMatchOptions = adminMatchOptions),
    )

    /**
     * Searches for a paginated result that matches the search criteria for AskOrders stored in the contract.  Throws
     * an exception if Provenance Blockchain communications fail.
     */
    fun searchAsks(searchRequest: ContractSearchRequest): ContractSearchResult<AskOrder> = queryContract(
        query = searchRequest.searchAsks(),
        typeReference = object : TypeReference<ContractSearchResult<AskOrder>>() {},
    )

    /**
     * Searches for a paginated result that matches the search criteria for AskOrders stored in the contract.  Returns
     * null if Provenance Blockchain communications fail.
     */
    fun searchAsksOrNull(searchRequest: ContractSearchRequest): ContractSearchResult<AskOrder>? = queryContractOrNull(
        query = searchRequest.searchAsks(),
        typeReference = object : TypeReference<ContractSearchResult<AskOrder>>() {},
    )

    /**
     * Searches for a paginated result that matches the search criteria for BidOrders stored in the contract.  Throws
     * an exception if Provenance Blockchain communications fail.
     */
    fun searchBids(searchRequest: ContractSearchRequest): ContractSearchResult<BidOrder> = queryContract(
        query = searchRequest.searchBids(),
        typeReference = object : TypeReference<ContractSearchResult<BidOrder>>() {},
    )

    /**
     * Searches for a paginated result that matches the search criteria for BidOrders stored in the contract.  Returns
     * null if Provenance Blockchain communications fail.
     */
    fun searchBidsOrNull(searchRequest: ContractSearchRequest): ContractSearchResult<BidOrder>? = queryContractOrNull(
        query = searchRequest.searchBids(),
        typeReference = object : TypeReference<ContractSearchResult<BidOrder>>() {},
    )

    /**
     * Fetches the core ContractInfo values for the smart contract, including information about the contract version,
     * its administrator, etc.  Throws an exception if Provenance Blockchain communications fail.
     */
    fun getContractInfo(): ContractInfo = queryContract(
        query = GetContractInfo(),
    )

    /**
     * Fetches the core ContractInfo values for the smart contract, including information about the contract version,
     * its administrator, etc.  Returns null if Provenance Blockchain communications fail.
     */
    fun getContractInfoOrNull(): ContractInfo? = queryContractOrNull(
        query = GetContractInfo(),
    )

    /**
     * Creates a new AskOrder in the contract for execution in matches with BidOrders.  Throws an exception if ask
     * validation fails, or if Provenance Blockchain communications fail.
     *
     * @param createAsk The message to send to the contract, detailing various aspects of the AskOrder.
     * @param signer Any implementation of Provenance Blockchain Inc.'s Signer interface for signing blockchain messages.
     * @param options Various tweaks to the way in which the constructed Provenance Blockchain transaction behaves.
     */
    fun createAsk(
        createAsk: CreateAsk,
        signer: Signer,
        options: BilateralBroadcastOptions = BilateralBroadcastOptions(),
    ): CreateAskResponse = executeContract(
        executeMsg = createAsk,
        signer = signer,
        options = options,
        funds = createAsk.ask.mapToFunds(),
    ).let { (event, data) ->
        CreateAskResponse(
            askId = event.stringAttribute("ask_id"),
            askFeeCharged = event.stringAttributeOrNull("ask_fee_charged"),
            askOrder = deserializeResponseData(data),
        )
    }

    /**
     * Alters an existing AskOrder in the contract.  The provided id in the constructor must match an existing ask order
     * for this function to pass validation and not throw an exception.  Throws an exception if Provenance Blockchain
     * communications fail.
     *
     * @param updateAsk The message to send to the contract, detailing the new shape of the updated AskOrder.
     * @param signer Any implementation of Provenance Blockchain Inc.'s Signer interface for signing blockchain messages.
     * @param options Various tweaks to the way in which the constructed Provenance Blockchain transaction behaves.
     */
    fun updateAsk(
        updateAsk: UpdateAsk,
        signer: Signer,
        options: BilateralBroadcastOptions = BilateralBroadcastOptions(),
    ): UpdateAskResponse = executeContract(
        executeMsg = updateAsk,
        signer = signer,
        options = options,
        funds = updateAsk.ask.mapToFunds(),
    ).let { (event, data) ->
        UpdateAskResponse(
            askId = event.stringAttribute("ask_id"),
            updatedAskOrder = deserializeResponseData(data),
        )
    }

    /**
     * Creates a new BidOrder in the contract for execution in matches with AskOrders.  Throws an exception if bid
     * validation fails, or if Provenance Blockchain communications fail.
     *
     * @param createBid The message to send to the contract, detailing various aspects of the BidOrder.
     * @param signer Any implementation of Provenance Blockchain Inc.'s Signer interface for signing blockchain messages.
     * @param options Various tweaks to the way in which the constructed Provenance Blockchain transaction behaves.
     */
    fun createBid(
        createBid: CreateBid,
        signer: Signer,
        options: BilateralBroadcastOptions = BilateralBroadcastOptions(),
    ): CreateBidResponse = executeContract(
        executeMsg = createBid,
        signer = signer,
        options = options,
        funds = createBid.bid.mapToFunds(),
    ).let { (event, data) ->
        CreateBidResponse(
            bidId = event.stringAttribute("bid_id"),
            bidFeeCharged = event.stringAttributeOrNull("bid_fee_charged"),
            bidOrder = deserializeResponseData(data),
        )
    }

    /**
     * Alters an existing BidOrder in the contract.  The provided id in the constructor must match an existing bid order
     * for this function to pass validation and not throw an exception.  Throws an exception if Provenance Blockchain
     * communications fail.
     *
     * @param updateBid The message to send to the contract, detailing the new shape of the updated BidOrder.
     * @param signer Any implementation of Provenance Blockchain Inc.'s Signer interface for signing blockchain messages.
     * @param options Various tweaks to the way in which the constructed Provenance Blockchain transaction behaves.
     */
    fun updateBid(
        updateBid: UpdateBid,
        signer: Signer,
        options: BilateralBroadcastOptions = BilateralBroadcastOptions(),
    ): UpdateBidResponse = executeContract(
        executeMsg = updateBid,
        signer = signer,
        options = options,
        funds = updateBid.bid.mapToFunds(),
    ).let { (event, data) ->
        UpdateBidResponse(
            bidId = event.stringAttribute("bid_id"),
            updatedBidOrder = deserializeResponseData(data),
        )
    }

    /**
     * Deletes an existing AskOrder in the contract.  Will initiate a refund of all held collateral (coin, marker, scope,
     * etc).  Throws an exception if the AskOrder cannot be found or if the if Provenance Blockchain communications fail.
     * Note: AskOrders are deleted (in most cases) after a match is made, so this should be expected to fail after a
     * match has executed.  Check the [ExecuteMatchResponse] on a successful match to determine if the ask has been
     * deleted.
     *
     * @param askId The unique id of the AskOrder to cancel.
     * @param signer Any implementation of Provenance Blockchain Inc.'s Signer interface for signing blockchain messages.
     * @param options Various tweaks to the way in which the constructed Provenance Blockchain transaction behaves.
     */
    fun cancelAsk(
        askId: String,
        signer: Signer,
        options: BilateralBroadcastOptions = BilateralBroadcastOptions(),
    ): CancelAskResponse = executeContract(
        executeMsg = CancelAsk(askId),
        signer = signer,
        options = options,
        funds = emptyList(),
    ).let { (event, data) ->
        CancelAskResponse(
            askId = event.stringAttribute("ask_id"),
            collateralReleased = event.booleanAttribute("collateral_released"),
            cancelledAskOrder = deserializeResponseData(data)
        )
    }

    /**
     * Deletes an existing BidOrder in the contract.  Will initiate a refund of all held coin.  Throws an exception if
     * the BidOrder cannot be found or if the if Provenance Blockchain communications fail. Note: BidOrders are deleted
     * after a match is made, so this should be expected to fail after a match has executed.  Check the
     * [ExecuteMatchResponse] on a successful match to determine if the bid has been deleted.
     *
     * @param bidId The unique id of the BidOrder to cancel.
     * @param signer Any implementation of Provenance Blockchain Inc.'s Signer interface for signing blockchain messages.
     * @param options Various tweaks to the way in which the constructed Provenance Blockchain transaction behaves.
     */
    fun cancelBid(
        bidId: String,
        signer: Signer,
        options: BilateralBroadcastOptions = BilateralBroadcastOptions(),
    ): CancelBidResponse = executeContract(
        executeMsg = CancelBid(bidId),
        signer = signer,
        options = options,
        funds = emptyList(),
    ).let { (event, data) ->
        CancelBidResponse(
            bidId = event.stringAttribute("bid_id"),
            cancelledBidOrder = deserializeResponseData(data),
        )
    }

    // IMPORTANT: The Signer used in this function must be the contract's admin account or the asker associated with the
    // match message's askId.
    /**
     * Matches an existing ask against an existing bid to execute a bilateral exchange of values (coin, marker, scope,
     * etc).  Throws an exception if match validation fails or if Provenance Blockchain communications fail.
     *
     * IMPORTANT: The Signer used in this function must be the contract's admin account or the asker associated
     * with the match message's askId.
     *
     * @param executeMatch The message to send to the contract, detailing the ask and bid to match, as well as if bids
     * that do not match the ask's requested quote will be accepted.
     * @param signer Any implementation of Provenance Blockchain Inc.'s Signer interface for signing blockchain messages.
     * @param options Various tweaks to the way in which the constructed Provenance Blockchain transaction behaves.
     */
    fun executeMatch(
        executeMatch: ExecuteMatch,
        signer: Signer,
        options: BilateralBroadcastOptions = BilateralBroadcastOptions(),
    ): ExecuteMatchResponse = executeContract(
        executeMsg = executeMatch,
        signer = signer,
        options = options,
        funds = emptyList(),
    ).let { (event) ->
        ExecuteMatchResponse(
            askId = event.stringAttribute("ask_id"),
            bidId = event.stringAttribute("bid_id"),
            askDeleted = event.booleanAttribute("ask_deleted"),
            bidDeleted = event.booleanAttribute("bid_deleted"),
            collateralReleased = event.booleanAttribute("collateral_released"),
        )
    }

    /**
     * Changes various settings that drive the contract's behavior.
     *
     * IMPORTANT: The Signer used in this function must be the contract's admin account, which can be found using the
     * [BilateralContractClient.getContractInfo] function.
     *
     * @param updateSettings The message to send to the contract, detailing the fields to be updated.
     * @param signer Any implementation of Provenance Blockchain Inc.'s Signer interface for signing blockchain messages.
     * @param options Various tweaks to the way in which the constructed Provenance Blockchain transaction behaves.
     */
    fun updateSettings(
        updateSettings: UpdateSettings,
        signer: Signer,
        options: BilateralBroadcastOptions = BilateralBroadcastOptions(),
    ): UpdateSettingsResponse = executeContract(
        executeMsg = updateSettings,
        signer = signer,
        options = options,
        funds = emptyList(),
    ).let { (event) ->
        UpdateSettingsResponse(
            newAdminAddress = event.stringAttributeOrNull("new_admin_address"),
            newAskFee = event.stringAttributeOrNull("new_ask_fee"),
            newBidFee = event.stringAttributeOrNull("new_bid_fee"),
        )
    }

    /**
     * Generates a [MsgExecuteContract] that will run the same functionality as the message sent in the
     * [BilateralContractClient.createAsk] function.
     *
     * @param createAsk The message to send to the contract, detailing various aspects of the AskOrder.
     * @param senderAddress The bech32 address of the account that will be required to sign the MsgExecuteContract
     * when processed in a Provenance Blockchain transaction.
     */
    fun generateCreateAskMsg(
        createAsk: CreateAsk,
        senderAddress: String,
    ): MsgExecuteContract = generateProtoExecuteMsg(
        executeMsg = createAsk,
        senderAddress = senderAddress,
        funds = createAsk.ask.mapToFunds(),
    )

    /**
     * Generates a [MsgExecuteContract] that will run the same functionality as the message sent in the
     * [BilateralContractClient.updateAsk] function.
     *
     * @param updateAsk The message to send to the contract, detailing the new shape of the updated AskOrder.
     * @param senderAddress The bech32 address of the account that will be required to sign the MsgExecuteContract
     * when processed in a Provenance Blockchain transaction.
     */
    fun generateUpdateAskMsg(
        updateAsk: UpdateAsk,
        senderAddress: String,
    ): MsgExecuteContract = generateProtoExecuteMsg(
        executeMsg = updateAsk,
        senderAddress = senderAddress,
        funds = updateAsk.ask.mapToFunds(),
    )

    /**
     * Generates a [MsgExecuteContract] that will run the same functionality as the message sent in the
     * [BilateralContractClient.createBid] function.
     *
     * @param createBid The message to send to the contract, detailing various aspects of the BidOrder.
     * @param senderAddress The bech32 address of the account that will be required to sign the MsgExecuteContract
     * when processed in a Provenance Blockchain transaction.
     */
    fun generateCreateBidMsg(
        createBid: CreateBid,
        senderAddress: String,
    ): MsgExecuteContract = generateProtoExecuteMsg(
        executeMsg = createBid,
        senderAddress = senderAddress,
        funds = createBid.bid.mapToFunds(),
    )

    /**
     * Generates a [MsgExecuteContract] that will run the same functionality as the message sent in the
     * [BilateralContractClient.updateBid] function.
     *
     * @param updateBid The message to send to the contract, detailing the new shape of the updated BidOrder.
     * @param senderAddress The bech32 address of the account that will be required to sign the MsgExecuteContract
     * when processed in a Provenance Blockchain transaction.
     */
    fun generateUpdateBidMsg(
        updateBid: UpdateBid,
        senderAddress: String,
    ): MsgExecuteContract = generateProtoExecuteMsg(
        executeMsg = updateBid,
        senderAddress = senderAddress,
        funds = updateBid.bid.mapToFunds(),
    )

    /**
     * Generates a [MsgExecuteContract] that will run the same functionality as the message sent in the
     * [BilateralContractClient.cancelAsk] function.
     *
     * @param askId The unique id of the AskOrder to cancel.
     * @param senderAddress The bech32 address of the account that will be required to sign the MsgExecuteContract
     * when processed in a Provenance Blockchain transaction.
     */
    fun generateCancelAskMsg(
        askId: String,
        senderAddress: String,
    ): MsgExecuteContract = generateProtoExecuteMsg(
        executeMsg = CancelAsk(askId),
        senderAddress = senderAddress,
        funds = emptyList(),
    )

    /**
     * Generates a [MsgExecuteContract] that will run the same functionality as the message sent in the
     * [BilateralContractClient.cancelBid] function.
     *
     * @param bidId The unique id of the BidOrder to cancel.
     * @param senderAddress The bech32 address of the account that will be required to sign the MsgExecuteContract
     * when processed in a Provenance Blockchain transaction.
     */
    fun generateCancelBidMsg(
        bidId: String,
        senderAddress: String,
    ): MsgExecuteContract = generateProtoExecuteMsg(
        executeMsg = CancelBid(bidId),
        senderAddress = senderAddress,
        funds = emptyList(),
    )

    /**
     * Generates a [MsgExecuteContract] that will run the same functionality as the message sent in the
     * [BilateralContractClient.executeMatch] function.
     *
     * IMPORTANT: The Signer used in this function must be the contract's admin account or the asker associated
     * with the match message's askId.
     *
     * @param executeMatch The message to send to the contract, detailing the ask and bid to match, as well as if bids
     * that do not match the ask's requested quote will be accepted.
     * @param senderAddress The bech32 address of the account that will be required to sign the MsgExecuteContract
     * when processed in a Provenance Blockchain transaction.
     */
    fun generateExecuteMatchMsg(
        executeMatch: ExecuteMatch,
        senderAddress: String,
    ): MsgExecuteContract = generateProtoExecuteMsg(
        executeMsg = executeMatch,
        senderAddress = senderAddress,
        funds = emptyList(),
    )

    /**
     * Generates a [MsgExecuteContract] that will run the same functionality as the message sent in the
     * [BilateralContractClient.updateSettings] function.
     *
     * IMPORTANT: The Signer used in this function must be the contract's admin account, which can be found using the
     * [BilateralContractClient.getContractInfo] function.
     *
     * @param updateSettings The message to send to the contract, detailing the fields to be updated.
     * @param senderAddress The bech32 address of the account that will be required to sign the MsgExecuteContract
     * when processed in a Provenance Blockchain transaction.
     */
    fun generateUpdateSettingsMsg(
        updateSettings: UpdateSettings,
        senderAddress: String,
    ): MsgExecuteContract = generateProtoExecuteMsg(
        executeMsg = updateSettings,
        senderAddress = senderAddress,
        funds = emptyList(),
    )

    /**
     * Converts a class that inherits from ContractExecuteMsg to a MsgExecuteContract.  This ensures
     */
    private fun generateProtoExecuteMsg(
        executeMsg: BilateralContractExecuteMsg,
        senderAddress: String,
        funds: List<Coin>,
        includeLogs: Boolean = true,
    ): MsgExecuteContract {
        val transactionDescription = executeMsg.toLoggingString()
        if (includeLogs) {
            logger.info("START: Generating tx for $transactionDescription")
        }
        return executeMsg.toExecuteMsg(
            objectMapper = objectMapper,
            contractAddress = contractAddress,
            senderBech32Address = senderAddress,
            funds = funds,
        ).also {
            if (includeLogs) {
                logger.info("END: Generating tx for $transactionDescription")
            }
        }
    }

    /**
     * Sends a ContractExecuteMsg to the resolved Metadata Bilateral Exchange contract address with the specified funds.
     * Throws exceptions if the PbClient is misconfigured or if a Provenance Blockchain or smart contract error occurs.
     */
    private fun executeContract(
        executeMsg: BilateralContractExecuteMsg,
        signer: Signer,
        options: BilateralBroadcastOptions,
        funds: List<Coin>,
    ): Pair<Event, TxResponse> {
        val transactionDescription = executeMsg.toLoggingString()
        logger.info("START: $transactionDescription")
        return pbClient.estimateAndBroadcastTx(
            txBody = generateProtoExecuteMsg(
                executeMsg = executeMsg,
                senderAddress = signer.address(),
                funds = funds,
                includeLogs = false,
            ).toAny().toTxBody(),
            signers = listOf(
                BaseReqSigner(
                    signer = signer,
                    sequenceOffset = options.sequenceOffset,
                    account = options.baseAccount
                )
            ),
            mode = options.broadcastMode,
            gasAdjustment = options.gasAdjustment,
        )
            .let { response -> response.singleWasmEvent() to response.txResponse }
            .also { logger.info("END: $transactionDescription") }
    }

    /**
     * Dynamic search function allowing for multiple behaviors in search responses and error handling, as configured
     * by each individual call.
     *
     * @param query The query to use as the JSON input to the contract.
     * @param throwExceptions If true, exceptions that occur in the request serialization/deserialization process will
     * be thrown.  If false, exceptions will be logged, but null will be returned.
     * @param typeReference An optional parameter that allows a response that requires complex deserialization with
     * generic type inference to be deserialized properly.  This is not relevant for must queries, but complex response
     * types that include lists of values or other generic parameters need this instead of a class reference for proper
     * deserialization inference.
     */
    private inline fun <T : BilateralContractQueryMsg, reified U : Any> queryContractBase(
        query: T,
        throwExceptions: Boolean,
        typeReference: TypeReference<U>?,
    ): U? = query.toLoggingString().let { queryDescription ->
        try {
            logger.info("START: $queryDescription")
            val responseDataBytes = pbClient.wasmClient.queryWasm(
                QueryOuterClass.QuerySmartContractStateRequest.newBuilder().also { req ->
                    req.address = contractAddress
                    req.queryData = query.toJsonByteString(objectMapper)
                }.build()
            ).data.toByteArray()
            if (typeReference != null) {
                objectMapper.readValue(responseDataBytes, typeReference)
            } else {
                objectMapper.readValue(responseDataBytes, U::class.java)
            }.also {
                logger.info("END: $queryDescription")
            }
        } catch (e: Exception) {
            if (throwExceptions) {
                throw e
            } else {
                logger.error("ERROR: $queryDescription", e)
                null
            }
        }
    }

    /**
     * Funnels input through queryContractBase, ensuring that no exceptions will be thrown if issues occur during
     * Provenance Blockchain communications, request serialization, or response deserialization.
     */
    private inline fun <T : BilateralContractQueryMsg, reified U : Any> queryContractOrNull(
        query: T,
        typeReference: TypeReference<U>? = null,
    ): U? = queryContractBase(
        query = query,
        throwExceptions = false,
        typeReference = typeReference,
    )

    /**
     * Funnels input through queryContractBase, allowing all exceptions to be thrown if they occur.
     */
    private inline fun <T : BilateralContractQueryMsg, reified U : Any> queryContract(
        query: T,
        typeReference: TypeReference<U>? = null,
    ): U = queryContractBase(
        query = query,
        throwExceptions = true,
        typeReference = typeReference,
    ) ?: throw NullContractResultException("Got null response from the Metadata Bilateral Exchange contract for query")

    /**
     * Processes a [TxResponse] produced via sending a [MsgExecuteContract] that targets the Metadata Bilateral Exchange
     * smart contract by parsing the response data into an expected model class.
     */
    private inline fun <reified T : Any> deserializeResponseData(response: TxResponse): T = try {
        objectMapper.readValue(response.executeContractDataToJsonBytes(), T::class.java)
    } catch (e: Exception) {
        throw ProvenanceEventParsingException("Failed to parse [${T::class.qualifiedName}] from response data: ${response.data}", e)
    }
}
