import { createFileRoute } from "@tanstack/react-router";
import { LoginPage } from "src/auth/LoginPage";

interface LoginSearch {
  redirect?: string;
}

export const Route = createFileRoute("/login")({
  component: RouteComponent,
  validateSearch: (search?: Record<string, unknown>): LoginSearch => {
    return {
      redirect: (search?.redirect as string) || undefined,
    };
  },
});

function RouteComponent() {
  return <LoginPage />;
}
