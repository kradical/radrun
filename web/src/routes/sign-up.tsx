import { createFileRoute } from "@tanstack/react-router";
import { SignUpPage } from "src/auth/SignUpPage";

export const Route = createFileRoute("/sign-up")({
  component: RouteComponent,
});

function RouteComponent() {
  return <SignUpPage />;
}
