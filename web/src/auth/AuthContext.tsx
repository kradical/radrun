import {
  createContext,
  type ReactNode,
  useCallback,
  useContext,
  useState,
} from "react";

const SESSION_EXPIRES_AT_KEY = "session_expires_at";

const getLocalSessionExpires = (): Date | undefined => {
  const strSessionExpires = localStorage.getItem(SESSION_EXPIRES_AT_KEY) ?? "";
  const numSessionExpires = Number.parseInt(strSessionExpires, 10);
  const dateSessionExpires = new Date(numSessionExpires);

  return Number.isNaN(dateSessionExpires.getTime())
    ? undefined
    : dateSessionExpires;
};

const setLocalSessionExpires = (date: Date): void => {
  localStorage.setItem(SESSION_EXPIRES_AT_KEY, date.getTime().toString());
};

interface AuthContextType {
  isLoggedIn: boolean;
  setSessionExpires: (d: Date) => void;
}

const initSessionExpires = getLocalSessionExpires();
const initIsLoggedIn =
  !!initSessionExpires && initSessionExpires.getTime() > Date.now();
const AuthContext = createContext<AuthContextType>({
  isLoggedIn: initIsLoggedIn,
  setSessionExpires: setLocalSessionExpires,
});

const AuthContextProvider = ({ children }: { children: ReactNode }) => {
  // TODO: make this expire when session expires
  const [sessionExpires, _setSessionExpires] = useState(
    getLocalSessionExpires(),
  );

  const setSessionExpires = useCallback((next: Date) => {
    setLocalSessionExpires(next);
    _setSessionExpires(next);
  }, []);

  const value = {
    isLoggedIn: !!sessionExpires && sessionExpires.getTime() > Date.now(),
    setSessionExpires,
  };

  return <AuthContext value={value}>{children}</AuthContext>;
};

const useIsLoggedIn = () => useContext(AuthContext).isLoggedIn;
const useSetSessionExpires = () => useContext(AuthContext).setSessionExpires;

export { AuthContextProvider, useIsLoggedIn, useSetSessionExpires };
