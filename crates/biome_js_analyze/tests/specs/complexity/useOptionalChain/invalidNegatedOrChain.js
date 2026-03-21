/* should generate diagnostics */

!foo || !foo.bar;
!foo || !foo.bar || !foo.bar.baz;
!foo.bar || !foo.bar.baz;
!foo || !foo.bar || !foo.bar.baz || !foo.bar.baz.buzz;
!foo || !foo[bar];
!foo || !foo[bar] || !foo[bar].baz;
!foo.bar || !foo.bar();
!a.b || !a.b();
(!foo || !foo.bar) && (!baz || !baz.bar);
!foo || !foo?.bar.baz;
