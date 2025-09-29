import { createFileRoute, redirect } from "@tanstack/react-router";
import { isAuthenticated } from "src/auth/AuthContext";

export const Route = createFileRoute("/")({
  component: Index,
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

function Index() {
  return (
    <div>
      <h3>Welcome Home!</h3>
    </div>
  );
}
