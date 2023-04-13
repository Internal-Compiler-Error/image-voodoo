/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  webpack(config) {
    config.experiments = {
      layers: true,
      asyncWebAssembly: true}
    return config
  }
}

module.exports = nextConfig
