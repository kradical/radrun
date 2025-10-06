import { currentUserQueryOptions } from "@api/client";
import { useSuspenseQuery } from "@tanstack/react-query";
import { createFileRoute, redirect } from "@tanstack/react-router";
import { isAuthenticated } from "src/auth/AuthContext";
import { queryClient } from "src/queryClient";

export const Route = createFileRoute("/profile")({
  component: Profile,
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
  loader: () => queryClient.ensureQueryData(currentUserQueryOptions),
});

function Profile() {
  const { data: user } = useSuspenseQuery(currentUserQueryOptions);

  return (
    <div>
      Hello {user.first_name} {user.last_name} from Profile!
    </div>
  );
}
