fn main() {
    topcoat::tailwind::BuildConfig::new().render().unwrap();
    topcoat::asset::BuildConfig::new().render().unwrap();
}
