const std = @import("std");
const wasi = std.os.wasi;

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();

    var args = WasiArgs.init(alloc);
    defer args.deinit();

    if (try args.load()) |err| {
        wasi.proc_exit(@intFromEnum(err));
    }

    puts("arg 1 = ");
    if (args.get(0)) |arg1| {
        puts(arg1);
    }
    puts("\n");
}

pub const WasiArgs = struct {
    alloc: std.mem.Allocator,
    args: [][]const u8,

    pub fn init(alloc: std.mem.Allocator) WasiArgs {
        return WasiArgs{
            .alloc = alloc,
            .args = &[_][]const u8{},
        };
    }

    pub fn load(self: *WasiArgs) !?wasi.errno_t {
        var argc: usize = 0;
        var argv_buf_size: usize = 0;

        const err = wasi.args_sizes_get(&argc, &argv_buf_size);
        if (err != wasi.errno_t.SUCCESS) return err;

        const argv_buf = try self.alloc.alloc(u8, argv_buf_size);
        defer self.alloc.free(argv_buf);

        const argv_ptrs = try self.alloc.alloc([*:0]u8, argc);
        defer self.alloc.free(argv_ptrs);

        const err2 = wasi.args_get(argv_ptrs.ptr, argv_buf.ptr);
        if (err2 != wasi.errno_t.SUCCESS) return err2;

        self.args = try self.alloc.alloc([]const u8, argc);

        var start: usize = 0;
        var i: usize = 0;
        while (i < argc) : (i += 1) {
            var len: usize = 0;
            while (argv_buf[start + len] != 0) : (len += 1) {}
            self.args[i] = argv_buf[start .. start + len];
            start += len + 1;
        }

        return null;
    }

    pub fn deinit(self: *const @This()) void {
        self.alloc.free(self.args);
    }

    pub fn get(self: *const @This(), index: usize) ?[]const u8 {
        if (index >= self.args.len) return null;
        return self.args[index];
    }
};

fn puts(msg: []const u8) void {
    var nwritten: usize = 0;

    _ = wasi.fd_write(
        1,
        &.{.{ .base = msg.ptr, .len = msg.len }},
        1,
        &nwritten,
    );
}
