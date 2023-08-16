package io.provenance.bilateral.execute

import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonNaming
import io.provenance.bilateral.interfaces.BilateralContractExecuteMsg
import io.provenance.bilateral.models.RequestDescriptor

/**
 * The top-level execute message that comprises a created ask.  See [Ask] for descriptions of each ask type.  The
 * sender of this message becomes the sole owner of the created [io.provenance.bilateral.models.AskOrder].
 *
 * @param ask Defines the type of ask to create.
 * @param descriptor Contains various options for an ask, universal to any request type.
 */
@JsonNaming(SnakeCaseStrategy::class)
@JsonTypeInfo(include = JsonTypeInfo.As.WRAPPER_OBJECT, use = JsonTypeInfo.Id.NAME)
@JsonTypeName("create_ask")
data class CreateAsk(
    val ask: Ask,
    val descriptor: RequestDescriptor? = null,
) : BilateralContractExecuteMsg {
    override fun toLoggingString(): String = "createAsk, ${ask.toLoggingStringSuffix()}"
}
