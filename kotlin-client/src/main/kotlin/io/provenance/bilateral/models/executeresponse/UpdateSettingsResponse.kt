package io.provenance.bilateral.models.executeresponse

/**
 * The data response returned after the contract settings are successfully updated.
 *
 * @param newAdminAddress Represents the new admin's bech32 address.  If the admin address was not changed, this value
 * will be null.
 * @param newAskFee Represents a formatted display of the new ask fee amount.  If the fee was not changed, this value
 * will be null.
 * @param newBidFee Represents a formatted display of the new bid fee amount.  If the fee was not changed, this value
 * will be null.
 */
data class UpdateSettingsResponse(
    val newAdminAddress: String?,
    val newAskFee: String?,
    val newBidFee: String?,
)
