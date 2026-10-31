// swift-tools-version: 5.7
import PackageDescription
let package = Package(
    name: "BazicalculatorUrlTools",
    products: [
        .library(name: "BazicalculatorUrlTools", targets: ["BazicalculatorUrlTools"]),
    ],
    targets: [
        .target(name: "BazicalculatorUrlTools", path: "swift/Sources/BazicalculatorUrlTools"),
        .executableTarget(name: "bazicalculator-url-check", dependencies: ["BazicalculatorUrlTools"], path: "swift/Check"),
    ]
)
