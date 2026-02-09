fn main() {
    // Generate Prisma client at build time
    #[cfg(debug_assertions)]
    println!("cargo:rerun-if-changed=prisma/schema.prisma");
}