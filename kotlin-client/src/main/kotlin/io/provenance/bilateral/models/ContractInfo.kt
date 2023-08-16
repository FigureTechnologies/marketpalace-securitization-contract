package io.provenance.bilateral.models

import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import com.fasterxml.jackson.databind.annotation.JsonNaming
import io.provenance.bilateral.serialization.CosmWasmUintToBigIntegerDeserializer
import java.math.BigInteger

/**
 * Internal settings on the Metadata Bilateral Exchange smart contract.  Some of these values can be altered using the
 * [io.provenance.bilateral.client.BilateralContractClient.updateSettings] function.
 *
 * @param admin The bech32 address of the contract's administrator account.  This account exclusively can execute
 * certain routes, and is intended to be used for cleanup purposes and/or automation.
 * @param bindName A name that the contract is bound to.  This allows for Provenance Blockchain name module lookups.
 * @param contractName A unique name that can be given to the contract for identification purposes in the case of
 * multiple instances of this contract being run simultaneously.
 * @param contractType An internal value to the contract that helps guard against incorrect migrations.  This value
 * cannot change after instantiation.
 * @param contractVersion The current version coded into the contract's Cargo.toml file.  This value is expected to
 * increment with each new migration applied to the contract.
 * @param createAskNhashFee The amount of nhash that is paid as a Provenance Blockchain fee when an account creates an
 * ask.  50% of this value is sent to the contract's admin account.
 * @param createBidNhashFee The amount of nhash that is paid as a Provenance Blockchain fee when an account creates a
 * bid.  50% of this value is sent to the contract's admin account.
 */
@JsonNaming(SnakeCaseStrategy::class)
data class ContractInfo(
    val admin: String,
    val bindName: String,
    val contractName: String,
    val contractType: String,
    val contractVersion: String,
    @JsonDeserialize(using = CosmWasmUintToBigIntegerDeserializer::class)
    val createAskNhashFee: BigInteger,
    @JsonDeserialize(using = CosmWasmUintToBigIntegerDeserializer::class)
    val createBidNhashFee: BigInteger,
)
