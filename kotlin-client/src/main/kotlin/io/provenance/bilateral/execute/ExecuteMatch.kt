package io.provenance.bilateral.execute

import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonNaming
import io.provenance.bilateral.interfaces.BilateralContractExecuteMsg
import io.provenance.bilateral.models.AdminMatchOptions

/**
 * An execute match call must be executed by the asker or the admin address.
 *
 * @param askId The unique identifier of the ask to match with.
 * @param bidId The unique identifier of the bid to match with.
 * @param adminMatchOptions Various options that alter the behavior of the matching mechanism. Using an incorrect
 * variant for the target match (ex: using marker trade admin options for matching two coin trades) will cause a
 * contract error.
 */
@JsonNaming(SnakeCaseStrategy::class)
@JsonTypeInfo(include = JsonTypeInfo.As.WRAPPER_OBJECT, use = JsonTypeInfo.Id.NAME)
@JsonTypeName("execute_match")
data class ExecuteMatch(
    val askId: String,
    val bidId: String,
    val adminMatchOptions: AdminMatchOptions? = null,
) : BilateralContractExecuteMsg {
    override fun toLoggingString(): String = "executeMatch, askId = [$askId], bidId = [$bidId], adminMatchOptions = [$adminMatchOptions]"
}
