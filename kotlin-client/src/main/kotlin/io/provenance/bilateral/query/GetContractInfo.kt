package io.provenance.bilateral.query

import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonNaming
import io.provenance.bilateral.interfaces.BilateralContractQueryMsg

/**
 * Fetches the internalized [io.provenance.bilateral.models.ContractInfo] value from the contract.  This value is
 * guaranteed to exist, and therefore this class does not require any input parameters.
 */
@JsonNaming(SnakeCaseStrategy::class)
@JsonTypeInfo(include = JsonTypeInfo.As.WRAPPER_OBJECT, use = JsonTypeInfo.Id.NAME)
@JsonTypeName("get_contract_info")
class GetContractInfo : BilateralContractQueryMsg {
    override fun toLoggingString(): String = "getContractInfo"
}
