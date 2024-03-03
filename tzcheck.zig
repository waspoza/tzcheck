const std = @import("std");
const time = @import("datetime.zig");
const timezones = @import("timezones.zig");

pub fn main() !void {
    const now = time.Datetime.now();
    const nowsecs: u64 = @intFromFloat(now.toSeconds());
    const date = try time.Datetime.create(2024, 2, 23, 2, 3, 32, 32, &timezones.Poland);
    const datesecs = @divFloor(date.toTimestamp(), 1000);

    std.debug.print("now: {}\n", .{nowsecs});
    std.debug.print("now: {}\n", .{std.time.timestamp()});
    std.debug.print("date: {}\n", .{datesecs});

    var general_purpose_allocator = std.heap.GeneralPurposeAllocator(.{}){};
    defer std.debug.assert(general_purpose_allocator.deinit() == .ok);
    const gpa = general_purpose_allocator.allocator();

    const result = try std.ChildProcess.run(.{
        .allocator = gpa,
        .argv = &[_][]const u8{
            "docker",
            "logs",
            "-n",
            "1",
            "octez-node",
        },
    });
    std.debug.print("{s}\n", .{result.stderr});
}
