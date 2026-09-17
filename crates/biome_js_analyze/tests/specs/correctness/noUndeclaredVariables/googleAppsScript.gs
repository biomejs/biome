// Google Apps Script service globals must be recognized (no diagnostics).
SpreadsheetApp.getActiveSpreadsheet();
Logger.log("hello");
DriveApp.getFiles();

// A genuinely undeclared variable must still be reported.
notAGlobal();
