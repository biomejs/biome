// With functions option configured, utility functions should be checked

// Should detect duplicates in cn()
cn("flex flex");
cn("p-4 m-2 p-4");

// Should detect duplicates in clsx()
clsx("hover:bg-blue hover:bg-blue");

// Should detect duplicates in cva()
cva("text-red text-red");

// Should detect duplicates in object keys
cn({ "flex flex": isActive });

// Valid: no duplicates
cn("flex p-4 m-2");
clsx("hover:bg-blue focus:bg-red");
