package io.provenance.bilateral.models

import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonNaming

/**
 * A payload that denotes whether or not a match can be made for a given [io.provenance.bilateral.models.AskOrder] and
 * [io.provenance.bilateral.models.BidOrder] to allow insight into the process without having to execute the match and
 * examine any successes or exceptions produced by it.
 *
 * @param askId The unique identifier for the ask in the match report.
 * @param bidId The unique identifier for the bid in the match report.
 * @param askExists If true, an [io.provenance.bilateral.models.AskOrder] with [askId] exists.
 * @param bidExists If true, a [io.provenance.bilateral.models.BidOrder] with [bidId] exists.
 * @param matchPossible If true, a match with the provided askId, bidId and specified adminMatchOptions is possible.
 * @param errorMessage If matchPossible is false, this value will be non-null with a detailed description of all issues
 * that are preventing the match from being made.
 */
@JsonNaming(SnakeCaseStrategy::class)
data class MatchReport(
    val askId: String,
    val bidId: String,
    val askExists: Boolean,
    val bidExists: Boolean,
    val matchPossible: Boolean,
    val errorMessage: String?,
)
