const wasi = @import("std").os.wasi;

pub fn main() void {
    puts("Hello, world!\n");
}

fn puts(msg: []const u8) void {
    var nwritten: usize = 0;

    _ = wasi.fd_write(
        1,
        &.{.{ .base = msg.ptr, .len = msg.len }},
        1,
        &nwritten,
    );
}
