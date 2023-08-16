package io.provenance.bilateral.execute

import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonNaming
import io.provenance.bilateral.interfaces.BilateralContractExecuteMsg
import io.provenance.bilateral.models.RequestDescriptor

/**
 * The top-level execute message that comprises an update for an ask.  This must target an existing ask by its id, and
 * will replace the contents of that ask with the new specified values.  If this changes the collateral for the ask, the
 * current collateral in the contract will be refunded to the asker.  This request can only be made by the owner of the
 * ask being modified.  See [Ask] for descriptions of each ask type.
 *
 * @param ask Defines the new structure of the ask.
 * @param descriptor Contains various options for an ask, universal to the request type.
 */
@JsonNaming(SnakeCaseStrategy::class)
@JsonTypeInfo(include = JsonTypeInfo.As.WRAPPER_OBJECT, use = JsonTypeInfo.Id.NAME)
@JsonTypeName("update_ask")
data class UpdateAsk(
    val ask: Ask,
    val descriptor: RequestDescriptor? = null,
) : BilateralContractExecuteMsg {
    override fun toLoggingString(): String = "updateAsk, ${ask.toLoggingStringSuffix()}"
}
