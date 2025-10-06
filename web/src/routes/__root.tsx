import { useQueryErrorResetBoundary } from "@tanstack/react-query";
import {
  createRootRoute,
  Link,
  Outlet,
  useRouter,
} from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";
import { useEffect } from "react";
import { useisAuthenticated } from "src/auth/AuthContext";

const RootLayout = () => {
  const isAuthenticated = useisAuthenticated();

  return (
    <>
      <div className="p-2 flex gap-2">
        {isAuthenticated && (
          <>
            <Link to="/" className="[&.active]:font-bold">
              Home
            </Link>{" "}
            <Link to="/about" className="[&.active]:font-bold">
              About
            </Link>{" "}
            <Link to="/profile" className="[&.active]:font-bold">
              Profile
            </Link>
          </>
        )}

        {!isAuthenticated && (
          <>
            <Link to="/login" className="[&.active]:font-bold">
              Login
            </Link>{" "}
            <Link to="/sign-up" className="[&.active]:font-bold">
              Sign Up
            </Link>
          </>
        )}
      </div>
      <hr />
      <Outlet />
      <TanStackRouterDevtools />
    </>
  );
};

export const Route = createRootRoute({
  component: RootLayout,
  errorComponent: ({ error }) => {
    const router = useRouter();
    const queryErrorResetBoundary = useQueryErrorResetBoundary();

    useEffect(() => {
      // Reset the query error boundary
      queryErrorResetBoundary.reset();
    }, [queryErrorResetBoundary]);

    return (
      <div>
        {error.message}
        <button
          onClick={() => {
            // Invalidate the route to reload the loader, and reset any router error boundaries
            router.invalidate();
          }}
          type="button"
        >
          retry
        </button>
      </div>
    );
  },
});
