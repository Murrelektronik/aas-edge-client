// Function to convert CamelCase to spaces and capitalize
export function camelCaseToSpaces(str) {
  return str
    // Handle special case where string starts with an acronym
    .replace(/^([A-Z]+)([A-Z][a-z])/g, "$1 $2")
    // Insert a space before sequences of capital letters if followed by lowercase letters
    .replace(/([A-Z]+)([A-Z][a-z])/g, "$1 $2")
    // Insert a space before single uppercase letters if not at the start
    .replace(/([a-z])([A-Z])/g, "$1 $2")
    // Capitalize the first letter of the string
    .replace(/^./, function(match) {
      return match.toUpperCase();
    });
}

// Function to convert CamelCase to kebab-case for paths
export function nameToPath(name) {
  return name.replace(/([a-z])([A-Z])/g, "$1-$2").toLowerCase();
}

export function submodelNameToEndpoint(submodelName) {
  return "/submodel/" +submodelName.toLowerCase().replace(/\s+/g, '-').trim();
}

export function findByKey(obj, keyToFind) {
  // Base case: if obj is not an object or array, return null
  if (obj === null || typeof obj !== 'object') {
    return null;
  }

  // Check if the current object has the key
  if (obj.hasOwnProperty(keyToFind)) {
    return obj[keyToFind];
  }

  // Recursively search for the key in arrays or nested objects
  for (let key of Object.keys(obj)) {
    const value = obj[key];
    if (typeof value === 'object') {
      const found = findByKey(value, keyToFind);
      if (found !== null) {
        return found;
      }
    }
  }

  // Key not found in current path
  return null;
}

// Find value by key but by passing the object, return value is not an object
export function findDirectValueByKey(obj, keyToFind) {
  // Base case: if obj is not an object or array, return null
  if (obj === null || typeof obj !== 'object') {
    return null;
  }

  // Check if the current object has the key and the value is not an object
  if (obj.hasOwnProperty(keyToFind) && !(obj[keyToFind] instanceof Object)) {
    return obj[keyToFind];
  }

  // Recursively search for the key in arrays or nested objects
  for (let key of Object.keys(obj)) {
    const value = obj[key];
    if (typeof value === 'object') {
      const found = findDirectValueByKey(value, keyToFind);
      if (found !== null) {
        return found;
      }
    }
  }

  // Key not found in current path
  return null;
}