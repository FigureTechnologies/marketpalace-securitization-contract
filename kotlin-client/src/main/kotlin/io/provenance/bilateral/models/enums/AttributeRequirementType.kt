package io.provenance.bilateral.models.enums

import com.fasterxml.jackson.annotation.JsonProperty

/**
 * Denotes an attribute requirement for use in creating a [io.provenance.bilateral.models.AttributeRequirement] in a
 * [io.provenance.bilateral.models.RequestDescriptor].  Each different type specifies a different behavior that is used
 * when the matching process executes.
 */
enum class AttributeRequirementType {
    /**
     * Specifies that all attributes set in the attribute requirement must exist on the other account for a match to be
     * accepted.
     */
    @JsonProperty("all") ALL,

    /**
     * Specifies that at least one of the attributes set in the attribute requirement must exist on the other account
     * for a match to be accepted.
     */
    @JsonProperty("any") ANY,

    /**
     * Specifies that none of the attributes set in the attribute requirement may exist on the other account for a match
     * to be accepted.
     */
    @JsonProperty("none") NONE,
}
