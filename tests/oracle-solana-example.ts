import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
// @ts-ignore
import {AnchorPriceOracle} from '../target/types/anchor_price_oracle';

describe('anchor-price-oracle', () => {

    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.AnchorPriceOracle as Program<AnchorPriceOracle>;

});
