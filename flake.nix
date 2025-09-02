{
  description = "Cross-platform app for uiua";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs =
    { self, nixpkgs, rust-overlay }:
    let
      supportedSystems = [ "x86_64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      overlays = [(import rust-overlay) ];
      pkgs = (import nixpkgs {
        system = "x86_64-linux";
        inherit overlays; 
        config = {
          android_sdk.accept_license = true;
          allowUnfree = true;
        };
        #config = {
        #android_sdk.accept_license = true;
        #allowUnfree = true;
        #};
      }
      );
    in
    {
      #packages = forAllSystems (system: {
      #  default = pkgs.callPackage ./default.nix { inherit pkgs; };
      #});
      devShells = forAllSystems (system: {
        default = pkgs.callPackage ./shell.nix { inherit pkgs; };
      });
    };
}
