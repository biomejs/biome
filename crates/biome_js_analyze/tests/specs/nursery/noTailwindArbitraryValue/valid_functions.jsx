/* should not generate diagnostics */
cn("hover:px-2 sm:text-lg");
clsx("w-4", "text-red-500");
classnames("bg-white");
cn({ "hover:px-2": isActive });
tw`w-4 text-red-500`;
tw.div`hover:px-2 sm:text-lg`;
tw.div`[&:nth-child(3)]:px-2`;
