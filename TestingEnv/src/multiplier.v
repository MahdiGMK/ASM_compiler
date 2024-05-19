
module multiplier(input clk , input reset , input [3:0]a , input [3:0]b , input [0:0]start , output reg [7:0]res , output reg [0:0]ready);
reg [1:0]currentState__42ef3fd;
reg [3 : 0]r0;
reg [7 : 0]r1;
reg [7 : 0]r2;
always @(posedge reset)
currentState__42ef3fd = 0;

always @(posedge clk) begin
if (currentState__42ef3fd == 0) begin
if (start) begin
r0 <= a;
r1 <= b;
r2 <= 0;
ready <= 0;
currentState__42ef3fd <= 1;
end else begin
currentState__42ef3fd <= 0;
end
end else
if (currentState__42ef3fd == 1) begin
r0 <= r0 >> 1;
r1 <= r1 << 1;
if (r0 == 0) begin
res <= r2;
ready <= 1;
currentState__42ef3fd <= 0;
end else begin
if (r0[0]) begin
r2 <= r2 + r1;
currentState__42ef3fd <= 1;
end else begin
currentState__42ef3fd <= 1;
end
end
end else begin
currentState__42ef3fd = 0;
end
end
endmodule