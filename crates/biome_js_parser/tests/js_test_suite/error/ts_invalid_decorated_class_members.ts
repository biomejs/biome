abstract class Test {
  @test constructor() {}
  @test declare prop;
  @test method();
  @test [index: string]: string;
  @test abstract method2();
  @test abstract get getter();
  @test abstract set setter(val);
}
