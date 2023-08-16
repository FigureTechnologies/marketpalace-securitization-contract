package io.provenance.bilateral.models

import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import com.fasterxml.jackson.databind.annotation.JsonNaming
import com.fasterxml.jackson.databind.annotation.JsonSerialize
import io.provenance.bilateral.models.enums.AttributeRequirementType
import io.provenance.bilateral.serialization.CosmWasmUTCOffsetDateTimeToTimestampSerializer
import io.provenance.bilateral.serialization.CosmWasmUTCTimestampToOffsetDateTimeDeserializer
import java.time.OffsetDateTime

/**
 * An optional set of fields to be applied to an [io.provenance.bilateral.models.AskOrder] or [io.provenance.bilateral.models.BidOrder]
 * that either tags them with informative fields or modifies their behavior.
 *
 * @param description A free-form text description of the ask.  For external examination purposes only.
 * @param effectiveTime The time at which the ask or bid order was created.  For external examination purposes only.
 * @param attributeRequirement Denotes specific attributes to be used in the matching process, based on the contents of
 * this value.
 */
@JsonNaming(SnakeCaseStrategy::class)
data class RequestDescriptor(
    val description: String? = null,
    @JsonSerialize(using = CosmWasmUTCOffsetDateTimeToTimestampSerializer::class)
    @JsonDeserialize(using = CosmWasmUTCTimestampToOffsetDateTimeDeserializer::class)
    val effectiveTime: OffsetDateTime? = null,
    val attributeRequirement: AttributeRequirement? = null,
)

/**
 * Specifies required attributes for a match to be made with the tagged ask or bid order.
 *
 * @param attributes The case-sensitive attribute names required for the ask or bid order to be executed in a match.
 * Ex: If attributes are provided for an [io.provenance.bilateral.models.AskOrder], then the matched [io.provenance.bilateral.models.BidOrder]
 * must have those attributes and meet the requirements of [requirementType].
 * @param requirementType Specifies the nature of the attribute requirement.  See [AttributeRequirementType] for the
 * different behaviors that this enum defines.
 */
@JsonNaming(SnakeCaseStrategy::class)
data class AttributeRequirement(
    val attributes: List<String>,
    val requirementType: AttributeRequirementType,
)
