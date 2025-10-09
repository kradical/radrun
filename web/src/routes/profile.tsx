import { currentUserQueryOptions } from "@api/client";
import { useSuspenseQuery } from "@tanstack/react-query";
import { createFileRoute, redirect } from "@tanstack/react-router";
import { isAuthenticated } from "src/auth/AuthContext";
import { queryClient } from "src/queryClient";
import { ProfilePage } from "src/user/ProfilePage";

export const Route = createFileRoute("/profile")({
  component: () => {
    const { data: user } = useSuspenseQuery(currentUserQueryOptions);
    return <ProfilePage user={user} />;
  },
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
