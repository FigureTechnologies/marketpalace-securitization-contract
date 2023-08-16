package io.provenance.bilateral.execute

import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonNaming
import com.fasterxml.jackson.databind.annotation.JsonSerialize
import io.provenance.bilateral.interfaces.BilateralContractExecuteMsg
import io.provenance.bilateral.serialization.CosmWasmBigIntegerToUintSerializer
import java.math.BigInteger

@JsonNaming(SnakeCaseStrategy::class)
@JsonTypeInfo(include = JsonTypeInfo.As.WRAPPER_OBJECT, use = JsonTypeInfo.Id.NAME)
@JsonTypeName("update_settings")
class UpdateSettings private constructor (val update: Body) : BilateralContractExecuteMsg {
    constructor(
        newAdminAddress: String? = null,
        newCreateAskNhashFee: BigInteger? = null,
        newCreateBidNhashFee: BigInteger? = null,
    ) : this(
        update = Body(
            newAdminAddress = newAdminAddress,
            newCreateAskNhashFee = newCreateAskNhashFee,
            newCreateBidNhashFee = newCreateBidNhashFee,
        )
    )

    /**
     * The request contents for updating contract settings.
     *
     * @param newAdminAddress A new admin for which to execute matches and perform other actions.  If set to null, the
     * existing admin will not be changed.  WARNING: This value can be arbitrarily set to any value.  If changed to an
     * address that is not controlled by any party, the contract will no longer be accessible for admin functionalities.
     * @param newCreateAskNhashFee An nhash fee to charge and send to the contract admin when new asks are created.  No
     * changes will be made if this value is null.  Note: This uses Provenance Blockchain custom fees, so 50% of the
     * charged nhash will be sent to the blockchain fee module, and the admin will receive the other 50%.
     * @param newCreateAskNhashFee An nhash fee to charge and send to the contract admin when new bids are created.  No
     * changes will be made if this value is null.  Note: This uses Provenance Blockchain custom fees, so 50% of the
     * charged nhash will be sent to the blockchain fee module, and the admin will receive the other 50%.
     */
    @JsonNaming(SnakeCaseStrategy::class)
    data class Body(
        val newAdminAddress: String?,
        @JsonSerialize(using = CosmWasmBigIntegerToUintSerializer::class)
        val newCreateAskNhashFee: BigInteger?,
        @JsonSerialize(using = CosmWasmBigIntegerToUintSerializer::class)
        val newCreateBidNhashFee: BigInteger?,
    )

    override fun toLoggingString(): String = "updateSettings, " +
        "newAdminAddress = [${update.newAdminAddress}], " +
        "newCreateAskNhashFee = [${update.newCreateAskNhashFee}], " +
        "newCreateBidNhashFee = [${update.newCreateBidNhashFee}]"
}
