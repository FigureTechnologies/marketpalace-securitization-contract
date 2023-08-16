package io.provenance.bilateral.execute

import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonNaming
import io.provenance.bilateral.interfaces.BilateralContractExecuteMsg

/**
 * A simple request to cancel an existing ask by referencing its unique id value.  This request must be executed by the
 * creator of the ask, or the administrator of the contract.  When an ask is cancelled, all of its collateral is returned
 * in totality to the asker.
 *
 * @param id The unique identifier for the ask to cancel.
 */
@JsonNaming(SnakeCaseStrategy::class)
@JsonTypeInfo(include = JsonTypeInfo.As.WRAPPER_OBJECT, use = JsonTypeInfo.Id.NAME)
@JsonTypeName("cancel_ask")
data class CancelAsk(val id: String) : BilateralContractExecuteMsg {
    override fun toLoggingString(): String = "cancelAsk, id = [$id]"
}
