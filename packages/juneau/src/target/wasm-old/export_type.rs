

// http://webassembly.github.io/spec/core/binary/modules.html#export-section
enum ExportType {
    func = 0x00,
    table = 0x01,
    mem = 0x02,
    global = 0x03
}