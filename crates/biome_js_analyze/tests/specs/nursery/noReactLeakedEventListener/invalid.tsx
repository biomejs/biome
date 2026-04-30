/* should generate diagnostics */
import { useEffect } from "react";

function Invalid1() {
  useEffect(() => {
    window.addEventListener("resize", handleResize);
  }, []);
}

function Invalid2() {
  useEffect(() => {
    const handleResize1 = () => {};
    const handleResize2 = () => {};
    window.addEventListener("resize", handleResize1);
    return () => {
      window.removeEventListener("resize", handleResize2);
    };
  }, []);
}

function Invalid3() {
  useEffect(() => {
    window.addEventListener("resize", () => {});
    return () => {
      window.removeEventListener("resize", () => {});
    };
  }, []);
}

function Invalid4() {
  useEffect(() => {
    window.addEventListener("resize", () => {}, { once: true });
    return () => {
      window.removeEventListener("resize", () => {});
    };
  }, []);
}

function Invalid5() {
  useEffect(() => {
    const handleResize1 = () => {};
    window.addEventListener("resize", handleResize1, { once: true });
  }, []);
}

function Invalid6() {
  useEffect(() => {
    window.addEventListener("resize", () => {}, { once: true });

    return () => {
      window.removeEventListener("resize", () => {}, { once: true });
    };
  }, []);
}

function Invalid7() {
  useEffect(() => {
    const handleResize = () => {};
    window.addEventListener("resize", handleResize, false);
    return () => {
      window.removeEventListener("resize", handleResize, true);
    };
  }, []);
}

function Invalid8() {
  useEffect(() => {
    const handleResize = () => {};
    window.addEventListener("resize", handleResize, true);
    return () => {
      window.removeEventListener("resize", handleResize, false);
    };
  }, []);
}

function Invalid9() {
  useEffect(() => {
    const handleResize = () => {};
    window.addEventListener("resize", handleResize);
    return () => {
      window.removeEventListener("resize", handleResize, true);
    };
  }, []);
}

function Invalid10() {
  useEffect(() => {
    const handleResize = () => {};
    window.addEventListener("resize", handleResize);
    return () => {
      window.removeEventListener("resize", handleResize, { capture: true });
    };
  }, []);
}

function Invalid11() {
  useEffect(() => {
    const handleResize = () => {};
    window.addEventListener("resize", handleResize, true);
    return () => {
      window.removeEventListener("resize", handleResize, { capture: false });
    };
  }, []);
}

function Invalid12() {
  useEffect(() => {
    const handleResize = () => {};
    window.addEventListener("resize", handleResize, { capture: true });
    return () => {
      window.removeEventListener("resize", handleResize, false);
    };
  }, []);
}

function Invalid13() {
  useEffect(() => {
    const handleResize = () => {};
    window.addEventListener("resize", handleResize, { capture: true });
    return () => {
      window.removeEventListener("resize", handleResize);
    };
  }, []);
}

function Invalid14() {
  useEffect(() => {
    const handleResize = () => {};
    window.addEventListener("resize", handleResize, { capture: true });
    return () => {
      window.removeEventListener("resize", handleResize, { capture: false });
    };
  }, []);
}

function Invalid15() {
  useEffect(() => {
    const handleResize = () => {};
    window.addEventListener("resize", handleResize, { capture: true });
    window.addEventListener("resize", handleResize, { capture: false });
    return () => {
      window.removeEventListener("resize", handleResize, { capture: false });
    };
  }, []);
}

function Invalid16() {
  useEffect(() => {
    const handleResize = () => {};
    window.addEventListener("resize", handleResize, { capture: true });
    window.addEventListener("resize", handleResize, { capture: false });
    window.addEventListener("resize", handleResize, { capture: false });
    return () => {
      window.removeEventListener("resize", handleResize, { capture: false });
    };
  }, []);
}

function Invalid17() {
  useEffect(() => {
    const options = { capture: true };
    const handleResize = () => {};
    window.addEventListener("resize", handleResize, options);
    return () => {
      window.removeEventListener("resize", handleResize);
    };
  }, []);
}

function Invalid18() {
  useEffect(() => {
    const options = { capture: true };
    const handleResize = () => {};
    window.addEventListener("resize", handleResize, { capture: options.capture });
    return () => {
      window.removeEventListener("resize", handleResize);
    };
  }, []);
}

function Invalid19() {
  const events = ["mousemove", "mousedown", "keydown", "scroll", "touchstart"];

  const handleActivity = () => {};

  useEffect(() => {
    events.forEach((evt) => {
      window.addEventListener(event, handleActivity);
    });

    return () => {
      events.forEach((event) => {
        window.removeEventListener(evt, handleActivity);
      });
    };
  }, []);

  return null;
}

function Invalid20() {
  const events = ["mousemove", "mousedown", "keydown", "scroll", "touchstart"];

  const handleActivity1 = () => {};
  const handleActivity2 = () => {};

  useEffect(() => {
    events.forEach((event) => {
      window.addEventListener(event, handleActivity1);
    });

    return () => {
      events.forEach((event) => {
        window.removeEventListener(event, handleActivity2);
      });
    };
  }, []);

  return null;
}

function Invalid21() {
  const events1 = ["mousemove", "mousedown", "keydown"];
  const events2 = ["keydown", "scroll", "touchstart"];

  const handleActivity = () => {};

  useEffect(() => {
    events1.forEach((event) => {
      window.addEventListener(event, handleActivity);
    });

    return () => {
      events2.forEach((event) => {
        window.removeEventListener(event, handleActivity);
      });
    };
  }, []);

  return null;
}

function Invalid22() {
  const events = ["mousemove", "mousedown", "keydown", "scroll", "touchstart"];

  const handleActivity = () => {};

  useEffect(() => {
    for (const evt of events) {
      window.addEventListener(event, handleActivity);
    }

    return () => {
      for (const event of events) {
        window.removeEventListener(evt, handleActivity);
      }
    };
  }, []);

  return null;
}

function Invalid23() {
  const events = ["mousemove", "mousedown", "keydown", "scroll", "touchstart"];

  const handleActivity1 = () => {};
  const handleActivity2 = () => {};

  useEffect(() => {
    for (const event of events) {
      window.addEventListener(event, handleActivity1);
    }

    return () => {
      for (const evt of events) {
        window.removeEventListener(evt, handleActivity2);
      }
    };
  }, []);

  return null;
}

function Invalid24() {
  const events = ["mousemove", "mousedown", "keydown", "scroll", "touchstart"];

  const handleActivity1 = () => {};
  const handleActivity2 = () => {};

  useEffect(() => {
    for (const event of events) {
      window.addEventListener(event, handleActivity1);
    }

    return () => {
      for (const [event] of events) {
        window.removeEventListener(event, handleActivity2);
      }
    };
  }, []);

  return null;
}

function Invalid25() {
  const events = ["mousemove", "mousedown", "keydown", "scroll", "touchstart"];

  const handleActivity1 = () => {};
  const handleActivity2 = () => {};

  useEffect(() => {
    for (const event of events) {
      window.addEventListener(event, handleActivity1);
    }

    return () => {
      for (const {event} of events) {
        window.removeEventListener(event, handleActivity2);
      }
    };
  }, []);

  return null;
}

function Invalid26() {
  useEffect(() => {
    (window.addEventListener as any)("resize", handleResize);
  }, []);
}
