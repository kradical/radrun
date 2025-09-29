import { createFileRoute, redirect } from "@tanstack/react-router";
import { isAuthenticated } from "src/auth/AuthContext";

export const Route = createFileRoute("/about")({
  component: About,
  beforeLoad: () => {
    if (!isAuthenticated()) {
      throw redirect({
        to: "/login",
        search: {
          redirect: location.href,
        },
      });
    }
  },
});

function About() {
  return <div>Hello from About!</div>;
}
