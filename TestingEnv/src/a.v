module test ();

  reg [3:0] address;
  reg [3:0] wData;
  reg write;
  wire [3:0] rwData;
  reg clk;
  reg reset;
  reg [3:0] a;
  reg [3:0] b;
  wire [7:0] mres;
  wire mready;
  reg mstart;

  assign rwData = write ? wData : 'bZ;

  ram _ram (
      .clk(clk),
      .reset(reset),
      .address(address),
      .data(rwData),
      .write(write)
  );

  multiplier _mult (
      .clk(clk),
      .reset(reset),
      .start(mstart),
      .a(a),
      .b(b),
      .res(mres),
      .ready(mready)
  );

  reg [3:0] addrs[7:0];
  reg [3:0] data1[7:0];
  reg [3:0] data2[7:0];
  integer i;

  always #1 clk = !clk;
  initial begin
    reset = 0;
    clk   = 0;
    $display("--------compiled ram test--------");

    #2;
    reset = 1;
    #1;
    reset = 0;
    #1;
    for (i = 0; i < 8; i = i + 1) begin
      addrs[i] = i;
      data1[i] = $urandom();
      data2[i] = $urandom();
    end

    write = 1;
    #2;
    for (i = 0; i < 8; i = i + 1) begin
      address = addrs[i];
      wData   = data1[i];
      #2;
    end

    for (i = 0; i < 8; i = i + 1) begin
      address = addrs[i];
      write   = 0;
      #2;
      if (rwData !== data1[i])
        $display(
            "Failed writing to / reading from ram[%x] - expected data=%b - got back data = %b",
            address,
            data1[i],
            rwData
        );
      else $display("successfully read value ram[%x] = %x == %x", address, rwData, data1[i]);
    end


    write = 1;
    #2;
    for (i = 0; i < 8; i = i + 1) begin
      address = addrs[i];
      wData   = data2[i];
      #2;
    end

    for (i = 0; i < 8; i = i + 1) begin
      address = addrs[i];
      write   = 0;
      #2;
      if (rwData !== data2[i])
        $display(
            "Failed writing to / reading from ram[%x] - expected data=%b - got back data = %b",
            address,
            data2[i],
            rwData
        );
      else $display("successfully read value ram[%x] = %x == %x", address, rwData, data2[i]);
    end
    $display("--------compiled multiplier test--------");

    reset = 1;
    #1;
    reset = 0;
    #1;
    for (i = 0; i < 8; i = i + 1) begin
      a = $urandom();
      b = $urandom();
      $display("%t : starting calculation %d * %d", $time, a, b);
      mstart = 1;
      #2;
      mstart = 0;
      wait (mready);
      $display("%t : calculation done %d * %d = %d", $time, a, b, mres);
      if (a * b !== mres) $display("wrong! should have been %d", a * b);
      else $display("correct!");
      #1;
    end

  end
endmodule
