import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
  id("com.android.library")
  kotlin("android")
}

android {
  namespace = "network.coblox.core"
  compileSdk = 35

  defaultConfig {
    minSdk = 26
    testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
  }

  sourceSets["main"].java.srcDir(layout.buildDirectory.dir("generated/uniffi"))
  sourceSets["test"].resources.srcDir(layout.buildDirectory.dir("native-test"))

  // Must match the Kotlin jvmTarget below: AGP defaults javac's target to 1.8,
  // which fails `compileDebugKotlin` with "Inconsistent JVM-target
  // compatibility detected" once Kotlin targets a newer bytecode level.
  compileOptions {
    sourceCompatibility = JavaVersion.VERSION_17
    targetCompatibility = JavaVersion.VERSION_17
  }
}

kotlin { compilerOptions { jvmTarget.set(JvmTarget.JVM_17) } }

dependencies {
  implementation("net.java.dev.jna:jna:5.16.0")
  testImplementation("junit:junit:4.13.2")
}

val workspaceRoot = rootProject.projectDir.resolve("../..").canonicalFile
val udl = workspaceRoot.resolve("core/coblox-ffi/src/coblox_ffi.udl")
val generatedBindings = layout.buildDirectory.dir("generated/uniffi")
val nativeTestDir = layout.buildDirectory.dir("native-test")

val generateUniFFIBindings by tasks.registering(Exec::class) {
  workingDir(workspaceRoot)
  inputs.file(udl)
  outputs.dir(generatedBindings)
  doFirst { generatedBindings.get().asFile.mkdirs() }
  commandLine(
    "cargo", "run", "--locked", "-p", "coblox-ffi", "--bin", "uniffi-bindgen", "--",
    "generate", udl.absolutePath, "--language", "kotlin", "--out-dir", generatedBindings.get().asFile.absolutePath
  )
}

val stageHostLibraryForTests by tasks.registering(Copy::class) {
  dependsOn(generateUniFFIBindings)
  val releaseLibrary = workspaceRoot.resolve("target/release/${System.mapLibraryName("coblox_ffi")}")
  inputs.file(releaseLibrary)
  from(releaseLibrary)
  into(nativeTestDir)
}

tasks.withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile>().configureEach {
  dependsOn(generateUniFFIBindings)
}
// `sourceSets["test"].resources.srcDir(nativeTestDir)` (above) makes AGP's
// processDebugUnitTestJavaRes read from nativeTestDir, but Gradle can't infer
// that from a plain srcDir() call, so task-graph validation flags it as an
// undeclared/implicit dependency unless that task is wired to
// stageHostLibraryForTests by name (AGP's merge-resources task type isn't a
// public Gradle type, so matching by `withType` is not reliable here).
afterEvaluate {
  tasks.named("processDebugUnitTestJavaRes") {
    dependsOn(stageHostLibraryForTests)
  }
}
tasks.withType<Test>().configureEach {
  dependsOn(stageHostLibraryForTests)
  doFirst { systemProperty("jna.library.path", nativeTestDir.get().asFile.absolutePath) }
}
