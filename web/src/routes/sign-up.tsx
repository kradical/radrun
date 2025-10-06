import { createFileRoute } from "@tanstack/react-router";
import { SignUpPage } from "src/auth/SignUpPage";

interface SignUpSearch {
  redirect?: string;
}

export const Route = createFileRoute("/sign-up")({
  component: RouteComponent,
  validateSearch: (search?: Record<string, unknown>): SignUpSearch => {
    return {
      redirect: (search?.redirect as string) || undefined,
    };
  },
});

function RouteComponent() {
  return <SignUpPage />;
}
