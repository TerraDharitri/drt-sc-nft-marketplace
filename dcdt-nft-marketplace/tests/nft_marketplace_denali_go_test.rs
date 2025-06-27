#[test]
fn auction_end_deadline_go() {
    numbat_wasm_debug::denali_go("denali/auction_end_deadline.scen.json");
}

#[test]
fn auction_end_max_bid_go() {
    numbat_wasm_debug::denali_go("denali/auction_end_max_bid.scen.json");
}

#[test]
fn auction_sell_all_end_deadline_go() {
    numbat_wasm_debug::denali_go("denali/auction_sell_all_end_deadline.scen.json");
}

#[test]
fn auction_sell_one_by_one_end_deadline_go() {
    numbat_wasm_debug::denali_go("denali/auction_sell_one_by_one_end_deadline.scen.json");
}

#[test]
fn auction_sft_sell_all_go() {
    numbat_wasm_debug::denali_go("denali/auction_sft_sell_all.scen.json");
}

#[test]
fn auction_sft_sell_one_by_one_go() {
    numbat_wasm_debug::denali_go("denali/auction_sft_sell_one_by_one.scen.json");
}

#[test]
fn auction_with_min_bid_delta_go() {
    numbat_wasm_debug::denali_go("denali/auction_with_min_bid_delta.scen.json");
}

#[test]
fn auction_with_min_bid_delta_buy_now_go() {
    numbat_wasm_debug::denali_go("denali/auction_with_min_bid_delta_buy_now.scen.json");
}

#[test]
fn auction_token_go() {
    numbat_wasm_debug::denali_go("denali/auction_token.scen.json");
}

#[test]
fn auction_with_start_time_go() {
    numbat_wasm_debug::denali_go("denali/auction_with_start_time.scen.json");
}

#[test]
fn bid_first_go() {
    numbat_wasm_debug::denali_go("denali/bid_first.scen.json");
}

#[test]
fn bid_max_go() {
    numbat_wasm_debug::denali_go("denali/bid_max.scen.json");
}

#[test]
fn bid_second_go() {
    numbat_wasm_debug::denali_go("denali/bid_second.scen.json");
}

#[test]
fn bid_sft_sell_all_first_go() {
    numbat_wasm_debug::denali_go("denali/bid_sft_sell_all_first.scen.json");
}

#[test]
fn bid_sft_sell_all_second_go() {
    numbat_wasm_debug::denali_go("denali/bid_sft_sell_all_second.scen.json");
}

#[test]
fn buy_sft_sell_one_by_one_go() {
    numbat_wasm_debug::denali_go("denali/buy_sft_sell_one_by_one.scen.json");
}

#[test]
fn bid_sft_sell_one_by_one_multiple_go() {
    numbat_wasm_debug::denali_go("denali/bid_sft_sell_one_by_one_multiple.scen.json");
}

#[test]
fn bid_with_token_whitelist_go() {
    numbat_wasm_debug::denali_go("denali/bid_with_token_whitelist.scen.json");
}

#[test]
fn buy_sft_sell_one_by_one_second_go() {
    numbat_wasm_debug::denali_go("denali/buy_sft_sell_one_by_one_second.scen.json");
}

#[test]
fn init_go() {
    numbat_wasm_debug::denali_go("denali/init.scen.json");
}

#[test]
fn invalid_bids_go() {
    numbat_wasm_debug::denali_go("denali/invalid_bids.scen.json");
}

#[test]
fn specific_token_auctioned_go() {
    numbat_wasm_debug::denali_go("denali/specific_token_auctioned.scen.json");
}

#[test]
fn view_functions_go() {
    numbat_wasm_debug::denali_go("denali/view_functions.scen.json");
}

#[test]
fn withdraw_go() {
    numbat_wasm_debug::denali_go("denali/withdraw.scen.json");
}

#[test]
fn withdraw_after_end_auction_go() {
    numbat_wasm_debug::denali_go("denali/withdraw_after_end_auction.scen.json");
}

#[test]
fn offer_token_go() {
    numbat_wasm_debug::denali_go("denali/offer_token.scen.json");
}

#[test]
fn offer_token_accept_go() {
    numbat_wasm_debug::denali_go("denali/offer_token_accept.scen.json");
}

#[test]
fn offer_token_with_active_auction_bid_after_offer_go() {
    numbat_wasm_debug::denali_go("denali/offer_token_with_active_auction_bid_after_offer.scen.json");
}

#[test]
fn offer_token_with_active_auction_no_bid_go() {
    numbat_wasm_debug::denali_go("denali/offer_token_with_active_auction_no_bid.scen.json");
}

#[test]
fn offer_token_with_active_auction_with_bid_go() {
    numbat_wasm_debug::denali_go("denali/offer_token_with_active_auction_with_bid.scen.json");
}

#[test]
fn offer_token_withdraw_reoffer_go() {
    numbat_wasm_debug::denali_go("denali/offer_token_withdraw_reoffer.scen.json");
}
