const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = std.Build.resolveTargetQuery(b, .{
        .cpu_arch = .wasm32,
        .os_tag = .wasi,
    });

    const mod = b.createModule(.{
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = .ReleaseFast,
    });

    const exe = b.addExecutable(.{ .name = "lo", .root_module = mod });
    b.installArtifact(exe);

    const run_cmd = b.addSystemCommand(&.{"wasmtime"});
    run_cmd.addArtifactArg(exe);

    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);
}
