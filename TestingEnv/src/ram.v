
module ram(input clk , input reset , input [3:0]address , inout [3:0]data , input [0:0]write);
reg [0:0]currentState__42ef3fd;
reg [3 : 0]data__92b72;
reg data_write_reg__20aac9b;
assign data = data_write_reg__20aac9b ? data__92b72 : 'bZ;
reg [3 : 0]mem[15 : 0];
always @(posedge reset)
currentState__42ef3fd = 0;

always @(posedge clk) begin
data_write_reg__20aac9b <= 0;
data__92b72 <= data;
if (currentState__42ef3fd == 0) begin
if (write) begin
mem[address] <= data;
currentState__42ef3fd <= 0;
end else begin
data__92b72 <= mem[address];
data_write_reg__20aac9b <= 1;
currentState__42ef3fd <= 0;
end
end else begin
currentState__42ef3fd = 0;
end
end
endmodule